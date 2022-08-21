use crate::{utils::blob_writer::{tease_file_exists, update_current_branch, read_head_commit, get_origin}, commands::goback::go_back, remote_req::clone_branch::clone_branch};
use std::path::Path;
use tease_common::write::bolb_writer::create_tease_file;


pub fn create_branch(name: String) -> () {
    let branch_head = format!("refs/heads/{}", name.to_string());
    let head_commit = read_head_commit();
    
    create_tease_file(Path::new(".tease").join(branch_head.to_string()).as_path(), head_commit);
    println!("Created branch {:?}", name.to_string());
    switch_to_branch(name);
}

pub fn switch_to_branch(name: String) -> () {
    let branch_head = format!("refs/heads/{}", name.to_string());
 
    if !tease_file_exists(branch_head.to_string()) {
        println!("Branch {:?} doesn't exist.", name.to_string());
        return ;
    }
    match update_current_branch(branch_head) {
        Ok(_) => {
            println!("Switched to branch {:?}", name.to_string());
            let head_commit = read_head_commit();
            go_back(head_commit);
        },
        Err(_) => println!("Couldn't switch to branch {:?}.", name.to_string()),
    }
}

pub fn create_from_remote(name: String) -> () {

    let origin = get_origin();
    if origin == "" {
        println!("Origin not set.");
        return ;
    }
    
    let temp_zip_res = clone_branch(name.to_string());
    if temp_zip_res.is_err() {
        println!("Failed to clone remote branch.");
        println!("Reason: {}", temp_zip_res.err().unwrap());
        return ;
    }

    let temp_zip_path = temp_zip_res.unwrap();

    tease_common::zip_utils::extraxt(temp_zip_path.to_string(), ".tease".to_string());

    switch_to_branch(name.to_string());
    let path = format!(".tease/refs/heads/{}-origin", name.to_string());
    let head_commit = read_head_commit();
    create_tease_file(Path::new(&path), head_commit);

    println!("Pulled branch {:?} from remote.", name.to_string());
}