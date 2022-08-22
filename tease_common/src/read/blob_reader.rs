use std::fmt::Display;
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
    
    // println!("{}", parts.len());
    
    if parts[1] == "#" {
        return ;
    }

    if parts.len() > 2 {
        if parts[1] != end_commit {
            trail.push(parts[1].to_string());
            trail_commit_history(root_folder, &parts[1].to_string(), end_commit, trail);            
        }
        if parts[2] != end_commit {
            trail.push(parts[2].to_string());
            trail_commit_history(root_folder, &parts[2].to_string(), end_commit, trail);
        }
    } else if parts[1] != end_commit {
        trail.push(parts[1].to_string());
        trail_commit_history(root_folder, &parts[1].to_string(), end_commit, trail);
    }
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

pub fn collect_objects_from_tree(root_folder: String, root_tree: String, objects: &mut Vec<String>) {
    let tree_content = read_object(&root_folder, &root_tree);
    let lines: Vec<&str> = tree_content.split("\n").collect();

    for line in lines {
        let parts: Vec<&str> = line.split(" ").collect();        
        if parts[0] == "blob" {
            objects.push(parts[2].to_string())
        }

        if parts[0] == "tree" {
            objects.push(parts[2].to_string());
            collect_objects_from_tree(root_folder.to_string(), parts[2].to_string(), objects);
        }
    }
}

#[derive(Default, Debug)]
pub struct IndexObject {
    pub sha1: String,
    pub path: String,
}

impl Display for IndexObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.path, self.sha1)
    }
}

fn format_path(curr_path: &Vec<String>, name: String) -> String {
    if curr_path.is_empty() {
        return name.to_string();
    }

    format!("{}/{}", curr_path.join("/"), name.to_string())
}

pub fn collect_from_tree(root_folder: String, root_tree: String) -> Vec<IndexObject> {
    let mut objects: Vec<IndexObject> = vec![];
    let mut trees: Vec<String> = vec![root_tree.to_string()];
    let mut visited: Vec<String> = vec![root_tree.to_string()];
    
    let mut curr_path: Vec<String> = vec![];

    while !trees.is_empty() {
        let tree = trees.last().unwrap().to_string();
        let tree_content = read_object(&root_folder, &tree.to_string());
        let lines: Vec<&str> = tree_content.split("\n").collect();
        let mut has_visited_tree = false;

        for line in lines {
            has_visited_tree = false;
            
            let parts: Vec<&str> = line.split(" ").collect();
            let path = format_path(&curr_path, parts[1].to_string());
            
            if parts[0] == "blob" && !visited.contains(&path.to_string()) {
                let blob = IndexObject {
                    sha1: parts[2].to_string(),
                    path: path.to_string()
                };
                objects.push(blob);
                visited.push(path.to_string());
            }

            if parts[0] == "tree" && !visited.contains(&path.to_string()){
                let tree = IndexObject {
                    sha1: parts[2].to_string(),
                    path: path.to_string() 
                };
                objects.push(tree);
                trees.push(parts[2].to_string());
                
                visited.push(path.to_string());
                curr_path.push(parts[1].to_string());
                has_visited_tree = true;
                break;
            }
        }
        if !has_visited_tree {
            curr_path.pop();
            trees.pop();
        }
    }

    objects

}

pub fn get_missing_objects(root_folder: String, incoming_objects: &Vec<String>, trail: &Vec<String>) -> Vec<String> {
    let mut objects: Vec<String> = vec![];

    for commit in trail.iter() {
        objects.push(commit.to_string());
        let tree = read_tree_from_commit(&root_folder.to_string(), &commit.to_string());
        objects.push(tree.to_string());
        collect_objects_from_tree(root_folder.to_string(), tree, &mut objects)
    }

    objects.sort();
    objects.dedup();

    incoming_objects.iter()
                    .filter(|&obj| !objects.contains(obj))
                    .map(|obj| obj.to_string())
                    .collect()
}

pub fn read_tree_from_commit(root_folder: &String, commit_sha1: &String) -> String {
    let commit_content = read_object(root_folder, commit_sha1);

    let mut parts: Vec<&str> = commit_content.split("\n").collect();

    parts = parts[0].split(" ").collect();
    parts[1].to_string()
}