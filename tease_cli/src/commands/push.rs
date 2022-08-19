use crate::{remote_req::{can_push::can_push, post_push::{setup_post, post_push}}, utils::blob_writer::has_added_files};
use std::fs::remove_file;

pub fn push () {

    if has_added_files() {
        println!("Commit added files before pushing.");
        return ;
    }

    let cp_res = can_push();
    if cp_res.is_err() {
        println!("{}", cp_res.err().unwrap());
        return ;
    }
    if setup_post(cp_res.unwrap()) {
        let res = post_push();
        if res.is_err() {
            println!("Couldn't push changes to remote origin.");
        }
        remove_file(".tease/temp_zip").unwrap();
    }

    println!("Pushed changes to remote origin.");
}