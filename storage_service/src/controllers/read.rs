use glob::glob;
use rocket::{serde::json::Json};
use serde::Serialize;
use tease_common::read::blob_reader::{IndexObject, safe_read_object, shallow_collect_from_tree, paths_to_string, read_object, trail_commits_all, CommitObject};

use crate::file_utils::read_branch_head;

#[derive(Debug, Serialize)]
pub struct CommitItem {
    pub sha1: String,
    pub date: u64,
    pub author: String,
    pub message: String,
    pub parents: Vec<String>,
}

fn new_commit_item(obj: &CommitObject) -> CommitItem {
    return CommitItem {
        sha1: obj.sha1.to_string(),
        date: obj.date,
        author: obj.author.to_string(),
        message: obj.message.to_string(),
        parents: obj.parents.to_vec()
    }
}

#[derive(Debug, Serialize)]
pub struct Commits {
    pub items: Vec<CommitItem>
}

#[derive(Debug, Serialize)]
pub struct TreeContent {
    pub items: Vec<TreeItem>
}

#[derive(Debug, Serialize)]
pub struct TreeItem {
    pub dtype: String,
    pub sha1: String,
    pub name: String
}

#[derive(Debug, Serialize)]
pub struct BlobContent {
    pub size: u64,
    pub content: String,
}


#[derive(Debug, Serialize)]
pub struct BranchContent {
    pub name: String, 
    pub tree_sha1: String,
    pub commit: CommitItem
}

fn new_tree_item(obj: &IndexObject) -> TreeItem {
    return TreeItem {
        dtype: obj.dtype.to_string(),
        sha1: obj.sha1.to_string(),
        name: obj.path.to_string()
    }
}

#[get("/<user>/<source_name>/commits/branch/<branch>")]
pub async fn read_commits(user: &str, source_name: &str, branch: &str) -> Option<Json<Commits>> {
    let root_folder = format!("source/{}/{}", user, source_name);

    let head_commit_res = read_branch_head(&root_folder, &branch.to_string());
    if head_commit_res.is_err() {
        return None;
    }
    let head_commit = head_commit_res.unwrap();

    let mut objects = trail_commits_all(root_folder, head_commit.to_string());
    objects.sort_by(|a, b| b.date.cmp(&a.date));
    objects.dedup_by(|a, b| a.sha1 == b.sha1);
    let items: Vec<CommitItem> = objects.iter()
                                      .map(|obj| new_commit_item(obj))
                                      .collect();

    Some(Json(Commits {items}))
}

#[get("/<user>/<source_name>/branch/<branch>")]
pub async fn read_branch(user: &str, source_name: &str, branch: &str) -> Option<Json<TreeContent>> {
    let root_folder = format!("source/{}/{}", user, source_name);

    let head_commit_res = read_branch_head(&root_folder, &branch.to_string());
    if head_commit_res.is_err() {
        return None;
    }
    let head_commit = head_commit_res.unwrap();
    let content_res = safe_read_object(&root_folder.to_string(), &head_commit);
    if content_res.is_err() {
        return None;
    }

    let content = content_res.unwrap();
    if !is_commit(content.to_string()) {
        return None;
    }

    let parts: Vec<&str> = content.split("\n").collect();
    let root_tree: &str = parts.get(0).unwrap().split(" ").collect::<Vec<&str>>().get(1).unwrap();

    let objects = shallow_collect_from_tree(root_folder, root_tree.to_string());
    let items: Vec<TreeItem> = objects.iter()
                                      .map(|obj| new_tree_item(obj))
                                      .collect();

    Some(Json(TreeContent {items}))
}


#[get("/<user>/<source_name>/tree/<sha1>")]
pub async fn read_tree(user: &str, source_name: &str, sha1: &str) -> Option<Json<TreeContent>> {
    let root_folder = format!("source/{}/{}", user, source_name);

    let content_res = safe_read_object(&root_folder.to_string(), &sha1.to_string());
    if content_res.is_err() {
        return None;
    }

    let content = content_res.unwrap();
    if is_blob(content.to_string()) || is_commit(content.to_string()) {
        return None;
    }

    let objects = shallow_collect_from_tree(root_folder, sha1.to_string());
    let items: Vec<TreeItem> = objects.iter()
                                      .map(|obj| new_tree_item(obj))
                                      .collect();

    Some(Json(TreeContent {items}))
}

#[get("/<user>/<source_name>/blob/<sha1>")]
pub async fn read_blob(user: &str, source_name: &str, sha1: &str) -> Option<Json<BlobContent>> {
    let root_folder = format!("source/{}/{}", user, source_name);

    let content_res = safe_read_object(&root_folder.to_string(), &sha1.to_string());
    if content_res.is_err() {
        return None;
    }
    
    let content = content_res.unwrap();
    if !is_blob(content.to_string()) {
        return None;
    }
    
    let parts: Vec<&str> = content.split("\0").collect();
    let info_parts: Vec<&str> = parts.get(0).unwrap().split(" ").collect();
    let size_res = info_parts.get(1).unwrap().parse::<u64>();
    if size_res.is_err() {
        return None;
    }
    Some(Json(BlobContent { size: size_res.unwrap(), content: parts.get(1).unwrap().to_string()}))
}

fn is_commit(content: String) -> bool {
    let parts: Vec<&str> = content.split("\n").collect();
    println!("{:?}", parts);
    if content.starts_with("tree") && parts.len() == 6 && parts.get(1).unwrap().to_owned().contains("parent") {
        return true
    }

    false
}

fn is_blob(content: String) -> bool {
    let parts: Vec<&str> = content.split("\0").collect();
    if parts.len() > 1 {
        return true;
    }

    false
}

#[get("/<user>/<source_name>/branch")]
pub async fn read_branches(user: &str, source_name: &str) -> Option<Json<Vec<BranchContent>>> {
    let root_folder = format!("source/{}/{}", user, source_name);

    let branch_names_res = glob(format!("{}/refs/heads/*", root_folder).as_str());
    if branch_names_res.is_err() {
        return None;
    }
    let branch_names = paths_to_string(branch_names_res.unwrap());
    let branches = branch_names.iter()
                               .map(|path| new_branch_content(root_folder.to_string(), path.to_string()))
                               .collect();

    Some(Json(branches))
}

fn new_branch_content(root_folder: String, path: String) -> BranchContent {
    let name = path.split("/").last().unwrap().to_string();
    let commit_res = read_branch_head(&root_folder, &name);
    let commit = commit_res.unwrap();
    let commit_content = read_object(&root_folder, &commit);
    
    let commit_lines: Vec<&str> = commit_content.split("\n").collect();
    let parents: Vec<&str> = commit_lines[1].split(" ").collect();
    let author: Vec<&str> = commit_lines[2].split(" ").collect();
    let date = author[2].parse::<u64>().unwrap();

    let commit = CommitItem {
        sha1: commit.to_string(),
        author: author[1].to_string(),
        date,
        message: commit_lines[5].to_string(),
        parents: parents[1..].iter().map(|s| s.to_string()).collect()
    };

    let lines: Vec<&str> = commit_content.split("\n").collect();
    let tree_sha1 = lines.get(0).unwrap().split(" ").last().unwrap().to_string();
    return BranchContent { name, tree_sha1, commit }
}