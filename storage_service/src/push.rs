use std::fs::{create_dir_all, remove_file};
use rocket::{Data, data::ToByteUnit, serde::{json::Json, Serialize, Deserialize}};

use crate::{zip_utils, jwt::JwtToken};

#[post("/<user>/<source_name>", data = "<src_data>")]
pub async fn push(user: &str, source_name: &str, src_data: Data<'_>) -> std::io::Result<String> {
    let dir_path = format!("source/{}/{}", user, source_name);
    let zip_path = format!("{}/temp_zip", dir_path);

    create_dir_all(&dir_path.to_string()).unwrap();

    src_data.open(128.kibibytes()).into_file(zip_path.to_string()).await?;
    zip_utils::extraxt(zip_path.to_string(), dir_path);
    remove_file(zip_path.to_string())?;
    
    Ok(format!("Uploaded files for {}/{}", user, source_name))
}

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
}

#[derive(Serialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct HasAccessRequest {
    user: String,
    owner: String,
    sourceName: String,
}

#[post("/<user>/<source_name>/can-push", format = "application/json", data="<src_data>")]
pub async fn can_push(jwt_token: JwtToken, user: &str, source_name: &str, src_data: Json<CanPushRequest>) -> Json<CanPushResponse> {
    let has_access_req = HasAccessRequest {
        user: jwt_token.email,
        owner: user.to_string(),
        sourceName: source_name.to_string()
    };

    let mut resp  = CanPushResponse {
        result: false
    };

    let res = has_access(has_access_req, jwt_token.token).await;
    if !res.is_ok() || res.unwrap() != true {
        return Json(resp); 
    }

    resp.result = true;
    Json(resp)
}


async fn has_access(req_body: HasAccessRequest, token: String) -> Result<bool, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let resp = client.post("http://localhost:8081/source/access")
        .header("Authorization", format!("Bearer {}", token))
        .json(&req_body)
        .send()
        .await
        .expect("Couldn't get response")
        .json::<rocket::serde::json::Value>()
        .await
        .expect("Couldn't decode...");
        
    println!("{:?}", resp);
    let result = resp.get("result").is_some();
    Ok(result)
}