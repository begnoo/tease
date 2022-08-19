use std::{fs::File};

use rocket::fs::NamedFile;
use tease_common::read::blob_reader::{read_tree_from_commit, collect_objects_from_tree, trail_commit_history};

use crate::{jwt::JwtToken, file_utils::read_branch_head};

#[get("/<user>/<source_name>", format = "application/json")]
pub async fn clone(
        _jwt_token: JwtToken,
        user: &str,
        source_name: &str,
    ) -> Option<NamedFile> {
    let root_folder = format!("source/{}/{}", user, source_name);
    let temp_zip_path = format!("{}/temp_zip", root_folder);
    let mut objects: Vec<String> = get_objects(root_folder.to_string(), "master".to_string()).iter()
                        .map(|obj| format!("{}/objects/{}", root_folder.to_string(), obj.to_string()))
                        .collect();
    objects.push(format!("{}/refs/heads/master", root_folder));
    
    let temp_zip = File::create(temp_zip_path.to_string()).unwrap();
    let res = tease_common::zip_utils::zip_files(objects, root_folder, temp_zip, zip::CompressionMethod::Stored);
    if res.is_err() {
        return None{};
    }

    NamedFile::open(temp_zip_path.to_string()).await.ok()
}

#[get("/<user>/<source_name>/clone/<branch_name>")]
pub async fn clone_branch(
        _jwt_token: JwtToken,
        user: &str,
        source_name: &str,
        branch_name: &str,
    ) -> Option<NamedFile> {
    let root_folder = format!("source/{}/{}", user, source_name);
    let temp_zip_path = format!("{}/temp_zip", root_folder);
    let mut objects: Vec<String> = get_objects(root_folder.to_string(), branch_name.to_string()).iter()
                        .map(|obj| format!("{}/objects/{}", root_folder.to_string(), obj.to_string()))
                        .collect();
    objects.push(format!("{}/refs/heads/{}", root_folder, branch_name.to_string()));

    let temp_zip = File::create(temp_zip_path.to_string()).unwrap();
    let res = tease_common::zip_utils::zip_files(objects, root_folder, temp_zip, zip::CompressionMethod::Stored);
    if res.is_err() {
        return None{};
    }

    NamedFile::open(temp_zip_path.to_string()).await.ok()
}


fn get_objects(root_folder: String, branch: String) -> Vec<String> {
    let mut objects: Vec<String> = vec![];

    let head_commit_res = read_branch_head(&root_folder, &branch.to_string());
    if head_commit_res.is_err() {
        return objects;
    }

    let head_commit = head_commit_res.unwrap();
    let mut commits: Vec<String> = vec![head_commit.to_string()];

    trail_commit_history(&root_folder, &head_commit, &"#".to_string(), &mut commits);
    commits.retain(|commit| commit != "");

    if commits.is_empty() {
        return objects;
    }

    objects.push(head_commit.to_string());
    for commit in commits.iter() {
        objects.push(commit.to_string());
        let tree = read_tree_from_commit(&root_folder, commit);
        objects.push(tree.to_string());
        collect_objects_from_tree(root_folder.to_string(), tree, &mut objects);
    }

    objects.sort();
    objects.dedup();
    objects
}