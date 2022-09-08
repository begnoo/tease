use serde::Serialize;

#[derive(Serialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct HasAccessRequest {
    pub user: String,
    pub owner: String,
    #[serde(rename = "sourceName")] 
    pub source_name: String,
}

pub async fn has_access(req_body: HasAccessRequest, token: String) -> bool {
    let client = reqwest::Client::new();
    let resp = client.post("http://localhost:8081/source/access")
        .header("Authorization", format!("Bearer {}", token))
        .json(&req_body)
        .send()
        .await;

    if resp.is_err() {
        return false;
    }

    let json_value = resp.unwrap().json::<rocket::serde::json::Value>()
        .await;
    
    if json_value.is_err() {
        return false;
    }
    
    let value = json_value.unwrap();
    let has_result = value.get("result").is_some();
    if !has_result {
        return  false;
    }
    let result = value.get("result").unwrap().as_bool().unwrap();
    result
}