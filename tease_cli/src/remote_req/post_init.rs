use std::fmt::Display;

use serde::{Deserialize};

use super::{
    get_clone::get_token, 
    responses::init::InitResponse, 
    requests::init::InitRequest
};

#[derive(Debug)]
pub struct InitError {
    pub message: String,
}

impl Display for InitError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

#[derive(Debug, Deserialize)]
struct ErrorResp {
    error: String
}

impl Display for ErrorResp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.error)
    }
}

#[tokio::main]
pub async fn post_init(email: String, name: String) -> Result<InitResponse, InitError> {
    let client = reqwest::Client::new();
    let token = get_token(name.to_string());

    let req_body = InitRequest {
        name,
        owner: email,
        visability: true
    };

    // println!("{:?}", req_body);

    let url = format!("http://localhost:8081/source");
    let resp = client.post(url)
        .header("Authorization", format!("Bearer {}", token))
        .json(&req_body)
        .send()
        .await;
    if resp.is_err() {
        return Err(InitError { message: "Couldn't decode response.".to_string() });
    }

    let json_resp = resp
        .unwrap()
        .json::<serde_json::Value>()
        .await;
    if json_resp.is_err() {
        return Err(InitError { message: "Couldn't parse response as json.".to_string() });
    }
    let json = json_resp.unwrap();
    // println!("{:?}", json);
    
    if json.is_array() {
        return Err(InitError{ message: "Validation error".to_string() });
    }

    if json.get("Owner").is_none() {
        return Err(InitError{ message: from_value_to_err_resp(json).error });
    }

    Ok(from_value_to_resp(json))
}

fn from_value_to_err_resp(value: serde_json::Value) -> ErrorResp {
    serde_json::from_value(value).unwrap()
}

fn  from_value_to_resp(value: serde_json::Value) -> InitResponse {
    serde_json::from_value(value).unwrap()
}