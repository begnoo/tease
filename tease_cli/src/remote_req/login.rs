use std::{fs::read_to_string, path::Path, io::{self, Write}};

use rpassword::read_password;
use serde::{Serialize, Deserialize};
use tease_common::write::bolb_writer::create_tease_file;

pub fn login() -> bool {
    let email = read_to_string(Path::new(".tease/user")).expect(&format!("Couldn't read user"));

    if email.trim() == "" {
        println!("Please set user.");
        return false;
    }

    print!("Enter password: ");
    _ = io::stdout().flush();
    let pass_res = read_password();
    if pass_res.is_err() {
        println!("Something went wrong while reading the password.");
        return false;
    }

    return post_login(email, pass_res.unwrap().trim_end().to_string());
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
pub async fn post_login(email: String, password: String) -> bool {

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
        create_tease_file(Path::new(".tease/bearer"), log_resp.token);
        return true;
    }
    println!("Couldn't login with provided credidentials");
    
    false
}

fn from_value_to_resp(value: serde_json::Value) -> LoginResponse {
    serde_json::from_value(value).unwrap()
}

pub fn get_token() -> String {
    let token = read_to_string(Path::new(".tease/bearer")).expect(&format!("Couldn't read token"));

    if token.trim() == "" {
        if !login() {
            return "".to_string();
        }
        return read_to_string(Path::new(".tease/bearer")).expect(&format!("Couldn't read token"));
    }

    token
}