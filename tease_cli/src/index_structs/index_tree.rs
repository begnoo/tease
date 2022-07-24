use sha1::{Sha1, Digest};
use std::collections::VecDeque;

use serde::{Deserialize, Serialize};
use serde_json::Result;

use crate::utils::blob_writer::compress_and_write_object;

#[derive(Serialize, Deserialize, Debug)]
pub struct IndexTreeNode {
    pub name: String,
    pub sha1_hash: String,
    pub children: Vec<IndexTreeNode>
}

pub fn _print_tree(root_node: &IndexTreeNode) -> Result<()> {
    let j = serde_json::to_string(root_node)?;
    println!("{}", j);

    Ok(())
}

pub fn extract_trees(tree: &IndexTreeNode) {
    let mut queue = VecDeque::new();
    
    queue.push_back(tree);

    while !queue.is_empty() {
        let current_node = queue.pop_front().expect("Queue is empty");

        if current_node.children.is_empty() {
            continue;
        }

        for child in current_node.children.iter() {
            queue.push_back(child);
        }

        let tree_content = get_tree_content(current_node);

        compress_and_write_object(tree_content.as_bytes(), current_node.sha1_hash.to_string())
            .expect(&format!("Couldn't save tree {:?}", current_node.name));
    }

}

pub fn add_to_tree(root_node: &mut IndexTreeNode, path: Vec<&str>, sha1: String) {
    
    if path.len() == 0 {
        return ;
    }

    if path.len() == 1 {
        root_node.children.push(
            IndexTreeNode {
                name: path[0].to_string(),
                sha1_hash: sha1.to_string(),
                children: Vec::new()
        });
        return ;
    }

    let mut found = false;

    
    for child in root_node.children.iter_mut() {
        if child.name == path[0].to_string() {
            found = true;
            add_to_tree(child, path[1..].to_vec(), sha1.to_string());
            set_hash_for_node(child);
        }
    }

    if !found {

        let mut new_node = IndexTreeNode {
            name: path[0].to_string(),
            sha1_hash: "".to_string(),
            children: Vec::new()
        };
        add_to_tree(&mut new_node, path[1..].to_vec(), sha1.to_string());
        
        set_hash_for_node(& mut new_node);
        root_node.children.push(new_node);
    }

}

fn get_tree_content(node: &IndexTreeNode) -> String {

    if node.children.is_empty() {
        return "".to_string();
    }

    let mut lines: Vec<String> = Vec::new();

    for child in node.children.iter() {
        let child_type = if child.children.is_empty() { "blob" } else { "tree" };
        let content = format!("{} {} {}", child_type, child.name, child.sha1_hash).to_string();
        lines.push(content);
    }

    lines.join("\n")
}

pub fn set_hash_for_node(node: & mut IndexTreeNode) {

    let tree_content = get_tree_content(node);
    
    if tree_content.is_empty() {
        return ;
    }

    let mut hasher = Sha1::new();
    hasher.update(tree_content.as_bytes());
    
    let binary_hash = hasher.finalize();
    let string_hash_vector: Vec<String> = binary_hash.iter().map(|n| format!("{:x?}", n)).collect();

    node.sha1_hash = string_hash_vector.join("");
}