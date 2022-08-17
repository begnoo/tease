use std::{fs::read_to_string, path::Path};

use glob::glob;
use serde::{Serialize, Deserialize};

use crate::utils::blob_writer::{get_current_branch, read_head_commit};

use tease_common::read::blob_reader::paths_to_string;

use super::login::login;

pub fn can_push() -> Result<CanPushResponse, CanPushError> {
    let email = read_to_string(Path::new(".tease/user"))
        .expect(&format!("Couldn't read user"));
    
    if email.trim() == "" {
        return Err(CanPushError{message: "Please set user before push/pull".to_string()});
    }

    let token = get_token();
    
    let cp_res = post_can_push(token);
    if cp_res.is_err() {
        return Err(cp_res.err().unwrap());
    }

    let cp = cp_res.unwrap();
    
    if cp.result == false && cp.diff.is_empty() {
        return Err(CanPushError{message: "You don't have anything to push...".to_string()});
    }

    if cp.result == false && cp.head_commit != read_head_commit() && !cp.diff.is_empty() {
        return Err(CanPushError{message: "Please pull, you are behind on commits...".to_string()});
    }

    Ok(cp)
}

#[derive(Serialize, Debug)]
pub struct CanPushRequest {
    pub branch: String,
    pub sha1: String,
    pub objects: Vec<String>,
}

#[derive(Deserialize)]
pub struct CanPushResponse {
    pub result: bool,
    pub diff: Vec<String>,
    pub head_commit: String,
    pub present: bool
}

#[derive(Debug, Clone)]
pub struct CanPushError {
    pub message: String
}

#[tokio::main]
async fn post_can_push(token: String) -> Result<CanPushResponse, CanPushError> {
    let branch = get_current_branch().split("/").last().unwrap().to_string();
    let branch_head = read_head_commit();
    let object_paths = glob(".tease/objects/*").expect("Failed to read glob pattern");
    let objects: Vec<String> = paths_to_string(object_paths).iter()
                                                .map(|obj| obj.split("/").last().unwrap().to_string())
                                                .collect();

    let req_body = CanPushRequest {
        branch,
        sha1: branch_head,
        objects
    };

    println!("{:?}", req_body);
    let client = reqwest::Client::new();
    let url = format!("{}/can-push", get_origin());
    let resp = client.post(url)
        .header("Authorization", format!("Bearer {}", token))
        .json(&req_body)
        .send()
        .await
        .expect("Couldn't get response")
        .json::<serde_json::Value>()
        .await
        .expect("Couldn't decode...");

    if resp.get("present").is_none() {
        return Err(CanPushError {message: "Something went wrong".to_string()});
    }
    let cp_res = from_value_to_resp(resp);

    Ok(cp_res)
}

fn from_value_to_resp(value: serde_json::Value) -> CanPushResponse {
    serde_json::from_value(value).unwrap()
}

fn get_token() -> String {
    let token = read_to_string(Path::new(".tease/bearer")).expect(&format!("Couldn't read token"));

    if token.trim() == "" {
        if !login() {
            return "".to_string();
        }
        return read_to_string(Path::new(".tease/bearer")).expect(&format!("Couldn't read token"));
    }

    token
}

fn get_origin() -> String {
    read_to_string(Path::new(".tease/origin")).expect(&format!("Couldn't read origin"))
}