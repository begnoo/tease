use std::fmt::Display;
use std::vec;
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

pub fn safe_read_object(root_folder: &String, object_name: &String) -> Result<String, std::io::Error> {
    let object_file_res = File::open(
            &Path::new(root_folder)
                    .join("objects")
                    .join(object_name));
    if object_file_res.is_err() {
        return Err(object_file_res.err().unwrap());
    }
    let object_file = object_file_res.unwrap();
    let mut decoder = ZlibDecoder::new(object_file);
    let mut decoded_str = String::new();
    decoder.read_to_string(&mut decoded_str).unwrap().to_string();

    Ok(decoded_str)
}

pub struct CommitObject {
    pub sha1: String,
    pub date: u64,
    pub author: String,
    pub message: String,
    pub parents: Vec<String>
}

impl Display for CommitObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Commit: {}\nAuthor: {}\nDate: {}\n\t{}", self.sha1, self.author, self.date, self.message)
    }
}


pub fn trail_commits_all(root_folder: String, starting_commit: String) -> Vec<CommitObject> {
    trail_commits_incl(root_folder, starting_commit, "#".to_string())
}

pub fn trail_commits_to(root_folder: String, starting_commit: String, end_commit: String) -> Vec<CommitObject> {
    let mut objects = trail_commits_incl(root_folder, starting_commit, end_commit.to_string());
    println!("{}", objects.len());
    
    let end_commit_pos = objects.iter().position(|obj| obj.sha1 == end_commit);
    if end_commit_pos.is_none() {
        return vec![];
    }
    objects.remove(end_commit_pos.unwrap());

    println!("{}", objects.len());
    
    objects
}

pub fn trail_commits_incl(root_folder: String, starting_commit: String, end_commit: String) -> Vec<CommitObject> {
    let mut trail: Vec<CommitObject> = vec![];
    let mut to_visit: Vec<String> = vec![starting_commit.to_string()];
    
    let mut current_commit: String;
    
    while !to_visit.is_empty() {
        current_commit = to_visit.pop().unwrap();

        if ((end_commit != "#" && current_commit == end_commit) || current_commit == "#") && to_visit.is_empty() {
            break;
        }

        if current_commit == "#" && !to_visit.is_empty() {
            continue;
        }

        let commit_content = read_object(&root_folder, &current_commit);
        let commit_lines: Vec<&str> = commit_content.split("\n").collect();
        let parents: Vec<&str> = commit_lines[1].split(" ").collect();
        let commit_obj = build_commit_obj(&current_commit, &commit_lines);
        
        trail.push(commit_obj);
        to_visit.push(parents[1].to_string());

        if parents.len() > 2 && parents[2] != "Starting" {
            to_visit.push(parents[2].to_string())
        }
    }

    trail
}

pub fn build_commit_obj(current_commit: &String, commit_lines: &Vec<&str>) -> CommitObject {
    let parents: Vec<&str> = commit_lines[1].split(" ").collect();
    let author: Vec<&str> = commit_lines[2].split(" ").collect();
    let date = author[2].parse::<u64>().unwrap();

    CommitObject {
        sha1: current_commit.to_string(),
        author: author[1].to_string(),
        date,
        message: commit_lines[5].to_string(),
        parents: parents[1..].iter().map(|s| s.to_string()).collect(),
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
    let history: Vec<String> = trail_commits_all(root_folder, branch_commit)
                                    .iter().map(|obj| obj.sha1.to_string())
                                    .collect();

    if history.contains(&new_commit) {
        return true;
    }

    false
}

pub fn collect_objects_from_tree(root_folder: String, root_tree: String) -> Vec<String> {
    collect_from_tree(root_folder, root_tree).iter()
                                             .map(|index_obj| index_obj.sha1.to_string())
                                             .collect()
}

#[derive(Default, Debug)]
pub struct IndexObject {
    pub sha1: String,
    pub path: String,
    pub dtype: String,
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
                    path: path.to_string(),
                    dtype: "blob".to_string()
                };
                objects.push(blob);
                visited.push(path.to_string());
            }

            if parts[0] == "tree" && !visited.contains(&path.to_string()){
                let tree = IndexObject {
                    sha1: parts[2].to_string(),
                    path: path.to_string(), 
                    dtype: "tree".to_string()
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

pub fn shallow_collect_from_tree(root_folder: String, root_tree: String) -> Vec<IndexObject> {
    let mut objects: Vec<IndexObject> = vec![];

    let tree_content = read_object(&root_folder, &root_tree.to_string());
    let lines: Vec<&str> = tree_content.split("\n").collect();

    for line in lines {
        let parts: Vec<&str> = line.split(" ").collect();
        let blob = IndexObject {
            sha1: parts[2].to_string(),
            path: parts[1].to_string(),
            dtype: parts[0].to_string()
        };
        objects.push(blob);
    }

    objects
}

pub fn get_missing_objects(root_folder: String, incoming_objects: &Vec<String>, trail: &Vec<String>) -> Vec<String> {
    let mut objects: Vec<String> = vec![];

    for commit in trail.iter() {
        objects.push(commit.to_string());
        let tree = read_tree_from_commit(&root_folder.to_string(), &commit.to_string());
        objects.push(tree.to_string());
    
        let mut collected_objects = collect_objects_from_tree(root_folder.to_string(), tree);
        objects.append(&mut collected_objects);
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