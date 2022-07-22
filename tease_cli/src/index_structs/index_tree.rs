use sha1::{Sha1, Digest};

use serde::{Deserialize, Serialize};
use serde_json::Result;

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

fn set_hash_for_node(node: & mut IndexTreeNode) {
    let mut lines: Vec<String> = Vec::new();

    if node.children.is_empty() {
        return ;
    }

    for child in node.children.iter() {
        let content = format!("{} {}", child.name, child.sha1_hash).to_string();
        lines.push(content);
    }
    
    let mut hasher = Sha1::new();
    hasher.update(lines.join("\n").as_bytes());
    
    let binary_hash = hasher.finalize();
    let string_hash_vector: Vec<String> = binary_hash.iter().map(|n| format!("{:x?}", n)).collect();

    node.sha1_hash = string_hash_vector.join("");
}