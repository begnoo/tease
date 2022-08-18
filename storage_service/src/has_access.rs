use serde::Serialize;

#[derive(Serialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct HasAccessRequest {
    pub user: String,
    pub owner: String,
    pub sourceName: String,
}

pub async fn has_access(req_body: HasAccessRequest, token: String) -> Result<bool, Box<dyn std::error::Error>> {
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
        
    let result = resp.get("result").is_some();
    Ok(result)
}