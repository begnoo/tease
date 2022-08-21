use std::{fmt::Display, io::Cursor, fs::read_to_string, path::Path};

pub struct CloneError {
    message: String
} 

impl Display for CloneError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

#[tokio::main]
pub async fn get_clone(origin: String, repo_name: String) -> Result<String, CloneError> {
    let client = reqwest::Client::new();
    let url = origin;
    let token = get_token(repo_name.to_string());
    let resp = client.get(url)
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await
        .expect("Couldn't get response");

    let file_res = std::fs::File::create("temp_zip");
    if file_res.is_err() {
        return Err(CloneError{message: "Couldn't create temp file for clone archive.".to_string()});
    }

    let bytes_res = resp.bytes().await;
    if file_res.is_err() {
        return Err(CloneError{message: "Couldn't parse bytes from response.".to_string()});
    }

    let mut content =  Cursor::new(bytes_res.unwrap());
    let res = std::io::copy(&mut content, &mut file_res.unwrap());
    if res.is_err() {
        return Err(CloneError{message: "Couldn't save temp file for clone archive.".to_string()});
    }

    Ok("temp_zip".to_string())
}

pub fn get_token(root_folder: String) -> String {
    let path = format!("{}/.tease/bearer", root_folder.to_string());
    let token = read_to_string(Path::new(&path)).expect(&format!("Couldn't read token"));
    
    token
}