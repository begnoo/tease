use rocket::{serde::json::Json, response::Result};
use serde::Serialize;
use tease_common::read::blob_reader::{read_object, collect_from_tree, IndexObject, safe_read_object};


#[derive(Debug, Serialize)]
pub struct RespError {
    pub error: String
}

#[derive(Debug, Serialize)]
pub struct TreeContent {
    pub items: Vec<TreeItem>
}

#[derive(Debug, Serialize)]
pub struct TreeItem {
    pub date_type: String,
    pub sha1: String,
    pub name: String
}

fn new_tree_item(obj: &IndexObject) -> TreeItem {
    return TreeItem {
        date_type: obj.data_type.to_string(),
        sha1: obj.sha1.to_string(),
        name: obj.path.to_string()
    }
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

    let objects = collect_from_tree(root_folder, sha1.to_string());
    let items: Vec<TreeItem> = objects.iter()
                                      .map(|obj| new_tree_item(obj))
                                      .collect();

    Some(Json(TreeContent {items}))
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