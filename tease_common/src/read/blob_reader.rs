use std::{fs::File, io::Read};
use std::path::Path;

use flate2::read::ZlibDecoder;
use glob::Paths;

pub fn read_object(root_folder: &String, object_name: &String) -> String {
    let object_file = File::open(
            &Path::new(root_folder)
                    .join("objects")
                    .join(object_name))
                    .expect(&format!("Coundn't read object {}", object_name)
            );
    let mut decoder = ZlibDecoder::new(object_file);
    let mut decoded_str = String::new();
    decoder.read_to_string(&mut decoded_str).unwrap().to_string();

    decoded_str
}

pub fn trail_commit_history(root_folder: &String, commit_sha1: &String, end_commit: &String, trail: &mut Vec<String>) {
    let commit_content = read_object(root_folder, commit_sha1);
    let mut parts: Vec<&str> = commit_content.split("\n").collect();
    parts = parts[1].split(" ").collect();
    
    if parts[1] == end_commit {
        return ;
    }

    if parts.len() > 2 {
        trail.push(parts[2].to_string());
        
    	trail_commit_history(root_folder, &parts[2].to_string(), end_commit, trail);
    }

    trail.push(parts[1].to_string());

    trail_commit_history(root_folder, &parts[1].to_string(), end_commit, trail);
}

pub fn paths_to_string(paths: Paths) -> Vec<String> {
    paths.into_iter()
        .map(|entry| entry.unwrap()
            .to_str()
            .unwrap()
            .to_string()
            .replace("\\", "/"))
        .collect()
}

pub fn contains_commit(root_folder: String, branch_commit: String, new_commit: String) -> bool {
    let mut history: Vec<String> = vec![branch_commit.to_string()];
    trail_commit_history(&root_folder, &branch_commit, &"#".to_string(), &mut history);

    if history.contains(&new_commit) {
        return true;
    }

    false
}


pub fn collect_objects_from_tree(root_tree: String, objects: &mut Vec<String>) {
    let tree_content = read_object(&".tease".to_string(), &root_tree);
    let lines: Vec<&str> = tree_content.split("\n").collect();

    for line in lines {
        let parts: Vec<&str> = line.split(" ").collect();        
        if parts[0] == "blob" {
            objects.push(parts[2].to_string())
        }

        if parts[0] == "tree" {
            objects.push(parts[2].to_string());
            collect_objects_from_tree(parts[2].to_string(), objects);
        }
    }
}