use std::{
    fs::read_to_string,
    path::Path, vec, fmt::Display
};

use crate::{
    utils::blob_writer::{
        get_current_branch,
        read_head_commit,
        read_origin_head_commit, get_origin,
    },
};

use tease_common::{
    read::blob_reader::{
        collect_objects_from_tree,
        read_tree_from_commit, contains_commit, trail_commits_incl,
    },
    write::bolb_writer::create_tease_file
};

use super::{login::get_token, responses::can_push::CanPushResponse, requests::can_push::CanPushRequest};

pub fn can_push() -> Result<CanPushResponse, CanPushError> {
    let email = read_to_string(Path::new(".tease/user"))
        .expect(&format!("Couldn't read user"));
    
    if email.trim() == "" {
        return Err(CanPushError{message: "Please set user before push/pull".to_string()});
    }
    
    let cp_res = post_can_push();
    if cp_res.is_err() {
        return Err(cp_res.err().unwrap());
    }

    let cp = cp_res.unwrap();
    // println!("{:?}", cp);

    if !cp.present {
        return Err(CanPushError{message: "No source initialized on given origin.".to_string()});
    }
    
    if !cp.result && cp.diff.is_empty() {
        return Err(CanPushError{message: "Nothing to push.".to_string()});
    }

    let origin_is_contained = cp.head_commit.to_string() == "" || contains_commit(".tease".to_string(), read_head_commit(), cp.head_commit.to_string());
    if !origin_is_contained && !cp.diff.is_empty() {
        return Err(CanPushError{message: "Please pull, you are behind on commits.".to_string()});
    }

    Ok(cp)
}

#[derive(Debug, Clone)]
pub struct CanPushError {
    pub message: String
}

impl Display for CanPushError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}


#[tokio::main]
async fn post_can_push() -> Result<CanPushResponse, CanPushError> {
    let branch = get_current_branch().split("/").last().unwrap().to_string();
    let branch_head = read_head_commit();
    let objects: Vec<String> = get_objects_to_send();

    let token = get_token().await;

    if token == "" {
        create_tease_file(Path::new(".tease/bearer"), "".to_string());
        return Err(CanPushError{message: "Authorization failed.".to_string()});
    }

    if objects.is_empty() {
        return Err(CanPushError {message: "Nothing to push.".to_string()});
    }

    let origin = get_origin(); 
    if origin == "" {
        return Err(CanPushError {message: "Set origin before pushing.".to_string()});
    }

    let req_body = CanPushRequest {
        branch,
        sha1: branch_head,
        objects
    };
    // println!("{:?}", req_body);
    let client = reqwest::Client::new();
    let url = format!("{}/can-push", origin);
    let resp = client.post(url)
        .header("Authorization", format!("Bearer {}", token))
        .json(&req_body)
        .send()
        .await
        .expect("Couldn't get response");
    
    if resp.status() == 401 {
        create_tease_file(Path::new(".tease/bearer"), "".to_string());
        return Err(CanPushError{message: "Authorization failed.".to_string()});
    }

    let json_resp = resp
        .json::<serde_json::Value>()
        .await
        .expect("Couldn't decode.");
    // println!("{:?}", json_resp);
    
    if json_resp.get("present").is_none() {
        return Err(CanPushError {message: "Something went wrong".to_string()});
    }
    let cp_res = from_value_to_resp(json_resp);
    // println!("{:?}", cp_res);
    Ok(cp_res)
}

fn from_value_to_resp(value: serde_json::Value) -> CanPushResponse {
    serde_json::from_value(value).unwrap()
}

fn get_objects_to_send() -> Vec<String> {
    let mut objects: Vec<String> = vec![]; 
    let local_head = read_head_commit();
    let mut origin_head = read_origin_head_commit();
    let root_folder = ".tease".to_string();

    if local_head == "# Starting commit" {
        return objects;
    }

    if origin_head == "" {
        origin_head = "#".to_string();
    }

    if local_head == origin_head {
        return objects;
    }

    let mut commits: Vec<String> = trail_commits_incl(root_folder.to_string(), local_head, origin_head)
                                    .iter()
                                    .map(|obj| obj.sha1.to_string())
                                    .collect();
    commits.retain(|commit| commit != "");

    if commits.is_empty() {
        return objects;
    }

    for commit in commits.iter() {
        objects.push(commit.to_string());
        let tree = read_tree_from_commit(&root_folder, commit);
        objects.push(tree.to_string());

        let mut collected_objects = collect_objects_from_tree(root_folder.to_string(), tree);
        objects.append(&mut collected_objects);
    }

    objects.sort();
    objects.dedup();
    objects
}