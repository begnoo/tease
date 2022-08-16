use std::{fs::{read_to_string, metadata, File}, path::Path, io::Read};

use flate2::read::ZlibDecoder;

pub fn read_object(object_name: &String) -> String {
    let object_file = File::open(&Path::new(".tease").join("objects").join(object_name))
        .expect(&format!("Coundn't read object {}", object_name));
    let mut decoder = ZlibDecoder::new(object_file);
    let mut decoded_str = String::new();
    decoder.read_to_string(&mut decoded_str).unwrap().to_string();

    decoded_str
}

pub fn get_current_branch() -> String {
    read_to_string(Path::new(".tease").join("HEAD"))
        .expect("Something went wrong reading the HEAD file")
}

pub fn read_head_commit() -> String {
    let current_ref_head = get_current_branch();
    let head_commit = read_to_string(Path::new(".tease").join(current_ref_head.to_string()))
        .expect(&format!("Couldn't read {}", current_ref_head));
    
    head_commit
}

pub fn tease_file_exists(path: String) -> bool {
    let md = metadata(Path::new(".tease").join(path));

    md.is_ok()
}

pub fn read_tree_from_commit(commit_sha1: &String) -> String {
    let commit_content = read_object(commit_sha1);

    let mut parts: Vec<&str> = commit_content.split("\n").collect();

    parts = parts[0].split(" ").collect();
    parts[1].to_string()
}

pub fn trail_commit_history(commit_sha1: &String, trail: &mut Vec<String>) {
    let commit_content = read_object(commit_sha1);
    let mut parts: Vec<&str> = commit_content.split("\n").collect();
    parts = parts[1].split(" ").collect();
    
    if parts[1] == "#" {
        return ;
    }

    if parts.len() > 2 {
        trail.push(parts[2].to_string());
    }

    trail.push(parts[1].to_string());

    trail_commit_history(&parts[1].to_string(), trail);
}