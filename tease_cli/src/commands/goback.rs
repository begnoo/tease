use std::{path::Path, fs::{metadata, remove_dir_all, remove_file}};

use crate::{utils::{blob_writer::{create_tease_file, create_tease_folder, create_index_file}, glob::get_all_repo_paths}, index_structs::index::{read_index, save_index}};

use super::{read::read_object, add::add_file};

pub fn go_back(commit_sha1: String) -> () {
    create_index_file(Path::new(".tease").join("index").as_path());

    let commit_content = read_object(&commit_sha1);

    let mut parts: Vec<&str> = commit_content.split("\n").collect();

    parts = parts[0].split(" ").collect();
    let root_tree = parts[1].to_string();
    delete_trail();

    traverse_commit_tree(root_tree.to_string(), "".to_string());
    update_index();
}

fn delete_trail() {
    let all_entries = get_all_repo_paths();
    println!("{:?}", all_entries);
    for entry in all_entries.iter() {
        // let path = Path::new(entry);
        let file_md = metadata(entry.to_string());
        match file_md {
            Ok(md) => {
                println!("{:?}", entry);
                println!("{:?}", md);
                if md.is_dir() {
                    remove_dir_all(entry.to_string()).unwrap();
                } else {
                    remove_file(entry.to_string()).unwrap();
                }
            }
            Err(_) => (),
        }
    }
}

fn update_index() {
    let mut index = read_index();

    for row in index.rows.iter_mut() {
        row.staging = 1;
    }

    save_index(index).expect("Couldn't save index while switching branches");
}

fn traverse_commit_tree(root_tree: String, prev_path: String) {
    let tree_content = read_object(&root_tree);
    let lines: Vec<&str> = tree_content.split("\n").collect();

    println!("{:?}", lines);

    for line in lines {
        let parts: Vec<&str> = line.split(" ").collect();
        println!("{:?}", parts);
        
        if parts[0] == "blob" {
            let blob_object = read_object(&parts[2].to_string());
            let blob_content = blob_object.split("\0").collect::<Vec<&str>>()[1];

            let new_file = if prev_path.is_empty() { parts[1].to_string() } else { vec![prev_path.to_string(), parts[1].to_string()].join("/") };
            create_tease_file(Path::new(&new_file.to_string()), blob_content.to_string());
            add_file(new_file.to_string()).expect("Couldn't recreate file.");
        }

        if parts[0] == "tree" {
            let new_folder = if prev_path.is_empty() { parts[1].to_string() } else { vec![prev_path.to_string(), parts[1].to_string()].join("/") };
            create_tease_folder(Path::new(&new_folder.to_string()));
            traverse_commit_tree(parts[2].to_string(), new_folder.to_string());
        }
    }
}