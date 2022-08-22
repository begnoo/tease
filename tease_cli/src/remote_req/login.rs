use std::{fs::read_to_string, path::Path, io::{self, Write}};

use rpassword::read_password;
use serde::{Serialize, Deserialize};
use tease_common::write::bolb_writer::create_tease_file;

pub async fn login() -> bool {
    let email = read_to_string(Path::new(".tease/user")).expect(&format!("Couldn't read user"));

    if email.trim() == "" {
        println!("Please set user.");
        return false;
    }
    let password = get_password();
    return post_login(email, password.to_string(), None).await
}

pub fn get_password() -> String {
    print!("Enter password: ");
    _ = io::stdout().flush();
    let pass_res = read_password();
    if pass_res.is_err() {
        println!("Something went wrong while reading the password.");
        return "".to_string();
    }
    return pass_res.unwrap();
}

pub fn login_with_prompt(root_folder: String) -> (String, bool) {
    print!("Enter email: ");
    _ = io::stdout().flush();
    let mut email = "".to_string();
    let email_res = io::stdin().read_line(&mut email);
    if email_res.is_err() {
        println!("Something went wrong while reading the email.");
        return (email, false);
    }

    let password = get_password();
    if password == "" {
        return (email, false);
    }

    (email.trim().to_string(), blocking_login(email.trim().to_string(), password.to_string(), Some(root_folder)))
}

#[derive(Serialize, Deserialize, Debug)]
struct LoginRequest {
    email: String,
    password: String
}


#[derive(Serialize, Deserialize, Debug)]
struct LoginResponse {
    token: String,
}

#[tokio::main]
pub async fn blocking_login(email: String, password: String, root_folder: Option<String>) -> bool {
    post_login(email, password, root_folder).await
}

// #[tokio::main]
pub async fn post_login(email: String, password: String, root_folder: Option<String>) -> bool {

    let req_body = LoginRequest {
        email,
        password
    };

    let client = reqwest::Client::new();
    let resp = client.post("http://localhost:8080/auth/login")
        // .header("Authorization", format!("Bearer {}", token))
        .json(&req_body)
        .send()
        .await
        .expect("Couldn't get response")
        .json::<serde_json::Value>()
        .await
        .expect("Couldn't decode.");
    
    if resp.get("token").is_some() {
        let log_resp = from_value_to_resp(resp);
        if root_folder.is_none() {
            create_tease_file(Path::new(".tease/bearer"), log_resp.token);
        } else {
            let path = format!("{}/.tease/bearer", root_folder.unwrap());
            create_tease_file(Path::new(&path), log_resp.token);
        }
        return true;
    }
    println!("Couldn't login with provided credidentials");
    
    false
}


fn from_value_to_resp(value: serde_json::Value) -> LoginResponse {
    serde_json::from_value(value).unwrap()
}

pub async fn get_token() -> String {
    let token = read_to_string(Path::new(".tease/bearer")).expect(&format!("Couldn't read token"));

    if token.trim() == "" {
        if !login().await {
            return "".to_string();
        }
        return read_to_string(Path::new(".tease/bearer")).expect(&format!("Couldn't read token"));
    }

    token
}