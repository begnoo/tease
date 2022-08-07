use sha1::{Sha1, Digest};

use crate::index_structs::commit::Commit;
use crate::utils::blob_writer::read_head_commit;
use crate::utils::blob_writer::update_head;
use crate::{index_structs::{index_tree::{add_to_tree, IndexTreeNode, extract_trees, set_hash_for_node}, index::{Index, read_index, flush_index}}, utils::blob_writer::compress_and_write_object};


pub fn commit(message: String) -> () {

    if !has_added_files() {
        println!("Nothing to commit.");
        return ;
    }

    let repo_tree = create_tree();
    extract_trees(&repo_tree);

    let new_commit = Commit {
        tree: repo_tree.sha1_hash,
        parent: read_head_commit(),
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

    compress_and_write_object(commit_content.as_bytes(), commit_sha1.to_string()).expect("Couldn't commit.");
    update_head(commit_sha1.to_string()).unwrap();
    flush_index();

    println!("Commited {}", commit_sha1);
}

fn has_added_files() -> bool {
    let index = read_index();

    index.rows.iter().any(|row| row.staging == 0 || row.staging == 2)
}

pub fn create_tree() -> IndexTreeNode {
    let index: Index = read_index();

    let mut root_node = IndexTreeNode {
        name: "root".to_string(),
        sha1_hash: "".to_string(),
        children: vec![]
    };
    
    for row in index.rows.iter() {
        if row.staging == 2 {
            continue;
        }
        let path_vec: Vec<&str> = row.file_name.split("/").collect();
        add_to_tree(&mut root_node, path_vec, row.blob_hash.to_string());
    }

    set_hash_for_node(& mut root_node);

    root_node
}