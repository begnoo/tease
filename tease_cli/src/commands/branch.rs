use crate::{utils::blob_writer::{create_tease_file, tease_file_exists, update_current_branch, read_head_commit}, commands::goback::go_back};
use std::path::Path;


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
        Err(_) => println!("Couldn't switch to branch {:?}", name.to_string()),
    }
}