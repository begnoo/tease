use std::{fs::{remove_dir_all, read_to_string}, path::Path, process::Command};

use tease_common::write::bolb_writer::create_tease_file;

use crate::remote_req::{get_clone::get_clone, login::login_with_prompt};

use super::create;

pub fn clone(origin: String) {

    let repo_name = origin.split("/").last().unwrap().to_string();
    create::create_repo(repo_name.to_string());

    let login_success = login_with_prompt(repo_name.to_string());
    if !login_success.1 {
        println!("Unauthorized.");
        return ;
    }

    let repo_name = origin.split("/").last().unwrap();
    let res = get_clone(origin.to_string(), repo_name.to_string());
    if res.is_err() {
        println!("Couldn't clone {}", origin.to_string());
        let rem_res = remove_dir_all(repo_name.to_string());
        if rem_res.is_err() {
            println!("Couldn't remove {}", repo_name.to_string());
        }
        return ;
    }
    let tease_root_folder = format!("{}/.tease", repo_name);
    tease_common::zip_utils::extraxt("temp_zip".to_string(), tease_root_folder);

    let origin_path = format!("{}/.tease/origin", repo_name);
    create_tease_file(Path::new(&origin_path.to_string()), origin.to_string());

    let maser_path = format!("{}/.tease/refs/heads/master", repo_name);
    let head_commit = read_to_string(Path::new(&maser_path.to_string()))
        .expect(&format!("Couldn't read {}", maser_path.to_string()));
    
    let maser_origin_path = format!("{}-origin", maser_path.to_string());
    create_tease_file(Path::new(&maser_origin_path.to_string()), head_commit.to_string());

    execute_go_back_shell(repo_name.to_string(), head_commit.to_string(), login_success.0);
}

fn execute_go_back_shell(repo_name: String, head_commit: String, email: String) {
    let go_back_res = Command::new("go_back".to_string())
        .arg(repo_name.to_string())    
        .arg(head_commit.to_string())
        .arg(email.to_string())
        .spawn();
    
    if go_back_res.is_err() {
        println!("Failed to start tease_cli go-back.");
    }

    let wait_res = go_back_res.unwrap().wait();
    if wait_res.is_err() {
        println!("Failed to execute tease_cli go-back.");
    }

    println!("Cloned source into folder {}", repo_name);
}