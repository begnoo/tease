use std::{fmt::Display, path::Path};

use tease_common::write::bolb_writer::create_tease_file;

use crate::utils::blob_writer::{get_origin, get_current_branch, read_origin_head_commit, read_head_commit};

use super::{login::get_token, responses::what_to_pull::ObjectCountResponse, requests::what_to_pull::ObjectCountRequest};

#[derive(Debug, Clone)]
pub struct WhatToPullError {
    pub message: String
}

impl Display for WhatToPullError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

#[tokio::main]
pub async fn what_to_pull() -> Result<ObjectCountResponse, WhatToPullError> {
    let client = reqwest::Client::new();
    let token = get_token().await;

    if token == "" {
        create_tease_file(Path::new(".tease/bearer"), "".to_string());
        return Err(WhatToPullError{message: "Authorization failed.".to_string()});
    }

    let req_body = ObjectCountRequest {
        branch: get_current_branch().split("/").last().unwrap().to_string(),
        past_origin_head: read_origin_head_commit(),
        current_head: read_head_commit(),
    };
    
    let url = format!("{}/what-to-pull", get_origin());
    let resp = client.post(url)
        .header("Authorization", format!("Bearer {}", token))
        .json(&req_body)
        .send()
        .await
        .expect("Couldn't get response");
    
    if resp.status() == 401 {
        create_tease_file(Path::new(".tease/bearer"), "".to_string());
        return Err(WhatToPullError{message: "Authorization failed.".to_string()});
    }

    let json_resp = resp
        .json::<serde_json::Value>()
        .await
        .expect("Couldn't decode.");

    if json_resp.get("objects").is_none() {
        return Err(WhatToPullError {message: "Couldn't convert json result to object.".to_string()});
    }
    
    Ok(from_value_to_resp(json_resp))
}

fn from_value_to_resp(value: serde_json::Value) -> ObjectCountResponse {
    serde_json::from_value(value).unwrap()
}