use rocket::serde::{Serialize, Deserialize, json::Json};
use tease_common::read::blob_reader::{contains_commit,  get_missing_objects};

use crate::{jwt::JwtToken, file_utils::read_branch_head, has_access::{HasAccessRequest, has_access}};
use std::fs::metadata;


#[derive(Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct CanPushRequest {
    pub branch: String,
    pub sha1: String,
    pub objects: Vec<String>,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct CanPushResponse {
    result: bool,
    diff: Vec<String>,
    head_commit: String,
    present: bool
}

#[post("/<user>/<source_name>/can-push", format = "application/json", data="<src_data>")]
pub async fn can_push(
        jwt_token: JwtToken,
        user: &str,
        source_name: &str,
        src_data: Json<CanPushRequest>
    ) -> Json<CanPushResponse> {

    let mut resp  = CanPushResponse {
        result: false,
        diff: vec![],
        head_commit: "".to_string(),
        present: false
    };

    let has_access_req = HasAccessRequest {
        user: jwt_token.email,
        owner: user.to_string(),
        sourceName: source_name.to_string()
    };

    let res = has_access(has_access_req, jwt_token.token).await;
    if res.is_err() || res.unwrap() != true {
        return Json(resp); 
    }

    let root_folder = format!("source/{}/{}", user.to_string(), source_name.to_string());
    let md = metadata(root_folder.to_string());
    if md.is_err() {
        resp.present = false;
        return Json(resp);
    } 

    let branch_head = read_branch_head(&root_folder, &src_data.branch);

    resp.present = true;

    let branch_commit = if branch_head.is_ok() { branch_head.unwrap() } else { "".to_string() }; 
    if branch_commit != "" && contains_commit(root_folder.to_string(), branch_commit.to_string(), src_data.sha1.to_string()) {
        return Json(resp); 
    }

    let missing_objects = get_missing_objects(root_folder, &src_data.objects);
    resp.diff = missing_objects;

    resp.head_commit = branch_commit;
    resp.result = resp.diff.len() != 0;
    Json(resp)
}