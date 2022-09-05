use rocket::serde::{Serialize, Deserialize, json::Json};
use tease_common::read::blob_reader::{contains_commit,  get_missing_objects, trail_commits_all, get_all_object_paths};

use crate::{jwt::JwtToken, file_utils::read_branch_head};
use std::fs::metadata;

use super::has_access::{HasAccessRequest, has_access};


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
        source_name: source_name.to_string()
    };

    let res = has_access(has_access_req, jwt_token.token).await;
    if res.is_err() || res.unwrap() != true {
        return Json(resp); 
    }

    let root_folder = format!("source/{}/{}", user.to_string(), source_name.to_string());
    let md = metadata(root_folder.to_string());
    if md.is_err() {
        return Json(resp);
    }

    resp.present = true;
    let branch_head = read_branch_head(&root_folder, &src_data.branch);
    let branch_commit = if branch_head.is_ok() { branch_head.unwrap() } else { "".to_string() };

    if branch_commit != "" {
        let trail: Vec<String> = trail_commits_all(root_folder.to_string(), branch_commit.to_string())
                                    .iter()
                                    .map(|obj| obj.sha1.to_string())
                                    .collect();
        let missing_objects = get_missing_objects(root_folder.to_string(), &src_data.objects, &trail);
        resp.diff = missing_objects;
    } else {
        resp.diff = if &src_data.branch == "master" { src_data.objects.to_vec() } else { new_branch_push(root_folder.to_string(), &src_data.objects) };
        resp.result = true;
        return Json(resp);
    }
    
    let is_contained = contains_commit(root_folder.to_string(), branch_commit.to_string(), src_data.sha1.to_string());
    if is_contained {
        return Json(resp); 
    }

    resp.head_commit = branch_commit.to_string();
    resp.result = resp.diff.len() != 0;
    Json(resp)
}

fn new_branch_push(root_folder: String, incoming_objects: &Vec<String>) -> Vec<String> {
    let what_we_have = get_all_object_paths(root_folder.to_string());
    let result: Vec<String> = incoming_objects.iter().filter(|obj| !what_we_have.contains(obj))
                                        .map(|obj| obj.to_string())
                                        .collect();
    println!("ovde samo se to desava");
    println!("{:?}", result);
    result    
}