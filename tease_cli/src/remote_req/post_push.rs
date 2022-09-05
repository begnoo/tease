use std::{path::Path, fs::File, fmt::Display};

use reqwest::{Client, Body};
use tease_common::write::bolb_writer::create_tease_file;
use tokio_util::codec::{FramedRead, BytesCodec};

use crate::utils::blob_writer::{get_current_branch, update_origin_head, read_head_commit, get_origin};

use super::{
    login::get_token, 
    responses::can_push::CanPushResponse
};
use tokio::fs::File as TokioFile;

pub fn setup_post (cp: CanPushResponse) -> bool {
    let mut objects: Vec<String> = cp.diff.iter()
                         .map( |obj| format!(".tease/objects/{}", obj))
                         .collect();

    objects.push(format!(".tease/{}", get_current_branch()));

    let temp_zip = File::create(".tease/temp_zip").unwrap();
    let res = tease_common::zip_utils::zip_files(objects, ".tease".to_string(), temp_zip, zip::CompressionMethod::Stored);
    if res.is_err() {
        println!("Couldn't archive objects to send.");
        return false;
    }
    
    println!("Archived files for push.");
    true
}

pub struct PushError {
    pub message: String
}

impl Display for PushError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

#[tokio::main]
pub async fn post_push () -> Result<(), PushError> {
    let file = TokioFile::open(".tease/temp_zip").await;
    if file.is_err() {
        return Err(PushError{message: "Couldn't access temp zip file.".to_string()});
    }

    let token = get_token().await;

    if token == "" {
        create_tease_file(Path::new(".tease/bearer"), "".to_string());
        return Err(PushError{message: "Authorization failed.".to_string()});
    }

    let client = Client::new();
    let resp = client.post(format!("{}/push", get_origin()))
        .header("Authorization", format!("Bearer {}", token))
        .body(file_to_body(file.unwrap()))
        .send()
        .await;

    if resp.is_err() {
        return Err(PushError{message: "Couldn't send push request.".to_string()});
    }

    if resp.unwrap().status() == 401 {
        create_tease_file(Path::new(".tease/bearer"), "".to_string());
        return Err(PushError{message: "Authorization failed.".to_string()});
    }
    
    let res = update_origin_head(read_head_commit());
    if res.is_err() {
        return Err(PushError{message: "Couldn't update origin head in refs.".to_string()});
    }

    Ok(())
}

fn file_to_body(file: TokioFile) -> Body {
    let stream = FramedRead::new(file, BytesCodec::new());
    let body = Body::wrap_stream(stream);
    body
}
