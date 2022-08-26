use std::{fs::{create_dir_all, remove_file}};
use rocket::{Data, data::ToByteUnit};

#[post("/<user>/<source_name>/push", data = "<src_data>")]
pub async fn push(user: &str, source_name: &str, src_data: Data<'_>) -> std::io::Result<String> {
    let dir_path = format!("source/{}/{}", user, source_name);
    let zip_path = format!("{}/temp_zip", dir_path);

    create_dir_all(&dir_path.to_string()).unwrap();

    src_data.open(128.kibibytes()).into_file(zip_path.to_string()).await?;
    tease_common::zip_utils::extraxt(zip_path.to_string(), dir_path);
    remove_file(zip_path.to_string())?;
    
    Ok(format!("Uploaded files for {}/{}", user, source_name))
}
