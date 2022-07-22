use sha1::{Sha1, Digest};

use crate::index_structs::{index_tree::{add_to_tree, IndexTreeNode, _print_tree, extract_trees, set_hash_for_node}, index::{Index, read_index}};

use super::add::compress_and_write_object;

pub struct Commit {
    tree: String,
    parent: String,
    author: String,
    commiter: String,
    message: String
}

pub fn commit(message: String) -> () {
    let repo_tree = create_tree();
    extract_trees(&repo_tree);

    let new_commit = Commit{
        tree: repo_tree.sha1_hash,
        parent: "".to_string(),
        author: "".to_string(),
        commiter: "".to_string(),
        message
    };

    let commit_content = format!("tree {}\nparent {}\nauthor {}\ncommiter {}\n\n{}", new_commit.tree, new_commit.parent, new_commit.author, new_commit.commiter, new_commit.message);

    let mut hasher = Sha1::new();
    hasher.update(commit_content.as_bytes());
    
    let binary_hash = hasher.finalize();
    let string_hash_vector: Vec<String> = binary_hash.iter().map(|n| format!("{:x?}", n)).collect();

    let commit_sha1 = string_hash_vector.join("");

    compress_and_write_object(commit_content.as_bytes(), commit_sha1).expect("Couldn't commit.")
}


pub fn create_tree() -> IndexTreeNode {
    let index: Index = read_index();

    let mut root_node = IndexTreeNode {
        name: "root".to_string(),
        sha1_hash: "".to_string(),
        children: vec![]
    };
    
    for row in index.rows.iter() {
        let path_vec: Vec<&str> = row.file_name.split("/").collect();
        add_to_tree(&mut root_node, path_vec, row.blob_hash.to_string());
    }

    set_hash_for_node(& mut root_node);

    _print_tree(&root_node);

    root_node
}