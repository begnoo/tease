use std::fs::remove_file;

use tease_common::read::blob_reader::{get_missing_objects, trail_commit_history};

use crate::{remote_req::{what_to_pull::what_to_pull, post_pull::post_pull}, utils::blob_writer::{get_current_branch, update_head, update_origin_head, read_head_commit, has_untracked_files}, commands::{merge::merge_commits, goback::go_back}};

pub fn pull() {

    if has_untracked_files() {
        println!("Commit or remove your untracked files before pulling.");
        return ;
    }

    let res_count_response = what_to_pull();
    if res_count_response.is_err() {
        println!("{}", res_count_response.err().unwrap());
        return ;
    }

    let count_response = res_count_response.unwrap();

    let head_commit = read_head_commit();
    
    let mut trail: Vec<String> = vec![];
    trail_commit_history(&".tease".to_string(), &head_commit.to_string(), &"#".to_string(), &mut trail);
    let missing_objects = get_missing_objects(".tease".to_string(), &count_response.objects, &trail);
    let missing_count = missing_objects.len();
    println!("missing_objects {:?}", missing_objects);
    
    if missing_count == 0 {
        println!("{} is already up-to-date.", get_current_branch());
        return ;
    }

    println!("Need to pull {} objects.", missing_count);
    
    let temp_path_res = post_pull(missing_objects);
    if temp_path_res.is_err() {
        println!("{}", temp_path_res.err().unwrap());
        return ;
    }

    let temp_path= temp_path_res.ok().unwrap();
    tease_common::zip_utils::extraxt(temp_path, ".tease".to_string());

    if count_response.merge_needed {
       merge_commits(read_head_commit(), count_response.origin_head.to_string());
    } else {
        go_back(count_response.origin_head.to_string());
        let mut update_res = update_head(count_response.origin_head.to_string());
        if update_res.is_err() {
            println!("Couldn't update branch head file while pulling.");
            return ;
        }
        update_res = update_origin_head(count_response.origin_head.to_string());
        if update_res.is_err() {
            println!("Couldn't update branch origin head file while pulling.");
            return ;
        }
    }

    let remove_res = remove_file(".tease/temp_zip");
    if remove_res.is_err() {
        println!("Couldn't remove temp file created while pulling.");
        return ;
    }

    println!("Pulled {} objects.", missing_count);

    if count_response.merge_needed {
        println!("Merge needed.");
    }
    
    println!("{} is now up-to-date.", get_current_branch());
}