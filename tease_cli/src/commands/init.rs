use std::{fs::remove_dir_all, path::Path};

use tease_common::write::bolb_writer::create_tease_file;

use crate::remote_req::{login::login_with_prompt, post_init::post_init};

use super::create;

pub fn init(name: String) {

    if Path::new(&name.to_string()).exists() {
        println!("File/Dir with same name already exists.");
        return ;
    }

    create::create_repo(name.to_string());

    let login_success = login_with_prompt(name.to_string());
    if !login_success.1 {
        println!("Unauthorized.");
        let rm_res = remove_dir_all(name.to_string());
        if rm_res.is_err() {
            println!("Failed in removing excess data.");
        }
        return ;
    }

    let init_res = post_init(login_success.0.to_string(), name.to_string());
    if init_res.is_err() {
        println!("Couldn't init source.");
        let rm_res = remove_dir_all(name.to_string());
        if rm_res.is_err() {
            println!("Failed in removing excess data.");
        }
    }

    let res = init_res.unwrap();
    let origin = format!("http://127.0.0.1:8000/source/{}/{}", res.owner, res.name);
    
    let origin_path = format!("{}/.tease/origin", res.name);
    let user_path = format!("{}/.tease/user", res.name);

    create_tease_file(Path::new(&origin_path), origin);
    create_tease_file(Path::new(&user_path), login_success.0);

}