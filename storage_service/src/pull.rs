use std::fs::File;

use rocket::{serde::{Deserialize, json::Json}, fs::NamedFile};

use crate::jwt::JwtToken;

#[derive(Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct PullRequest {
    pub objects: Vec<String>,
}

#[post("/<user>/<source_name>/pull", format = "application/json", data="<src_data>")]
pub async fn pull(
        _jwt_token: JwtToken,
        user: &str,
        source_name: &str,
        src_data: Json<PullRequest>
    ) -> Option<NamedFile> {
    let root_folder = format!("source/{}/{}", user, source_name);
    let temp_zip_path = format!("{}/temp_zip", root_folder);
    let objects: Vec<String> = src_data.objects.iter()
                        .map( |obj| format!("{}/objects/{}", root_folder.to_string(), obj))
                        .collect();
    let temp_zip = File::create(temp_zip_path.to_string()).unwrap();
    let res = tease_common::zip_utils::zip_files(objects, root_folder, temp_zip, zip::CompressionMethod::Stored);
    if res.is_err() {
        return None{};
    }

    NamedFile::open(temp_zip_path.to_string()).await.ok()
}