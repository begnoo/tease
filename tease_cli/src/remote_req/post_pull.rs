use std::{path::Path, io::Cursor, fmt::Display};

use tease_common::write::bolb_writer::create_tease_file;

use crate::utils::blob_writer::get_origin;

use super::{login::get_token, requests::pull::PullRequest};

pub struct PullError {
    pub message: String,
}

impl Display for PullError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

#[tokio::main]
pub async fn post_pull(objects: Vec<String>) -> Result<String, PullError> {
    let client = reqwest::Client::new();
    let token = get_token().await;

    if token == "" {
        create_tease_file(Path::new(".tease/bearer"), "".to_string());
        return Err(PullError{message: "Authorization failed.".to_string()});
    }

    let req_body = PullRequest {
        objects
    };

    let url = format!("{}/pull", get_origin());
    let resp = client.post(url)
        .header("Authorization", format!("Bearer {}", token))
        .json(&req_body)
        .send()
        .await
        .expect("Couldn't get response");
    
    if resp.status() == 401 {
        create_tease_file(Path::new(".tease/bearer"), "".to_string());
        return Err(PullError{message: "Authorization failed.".to_string()});
    }

    let file_res = std::fs::File::create(".tease/temp_zip");
    if file_res.is_err() {
        return Err(PullError{message: "Couldn't create temp file for pull archive.".to_string()});
    }

    let bytes_res = resp.bytes().await;
    if file_res.is_err() {
        return Err(PullError{message: "Couldn't parse bytes from response.".to_string()});
    }

    let mut content =  Cursor::new(bytes_res.unwrap());
    let res = std::io::copy(&mut content, &mut file_res.unwrap());
    if res.is_err() {
        return Err(PullError{message: "Couldn't save temp file for pull archive.".to_string()});
    }

    Ok(".tease/temp_zip".to_string())
}