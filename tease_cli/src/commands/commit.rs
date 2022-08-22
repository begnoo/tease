use std::fs::read_to_string;
use std::path::Path;

use sha1::{Sha1, Digest};

use crate::index_structs::commit::Commit;
use crate::index_structs::index::is_merging;
use crate::utils::blob_writer::has_added_files;
use crate::utils::blob_writer::read_head_commit;
use crate::utils::blob_writer::update_head;
use crate::{index_structs::{index_tree::{add_to_tree, IndexTreeNode, write_trees, set_hash_for_node}, index::{Index, read_index, flush_index}}, utils::blob_writer::compress_and_write_object};

use chrono::UTC;

pub fn commit(message: String) -> () {
    let index = read_index();

    if !has_added_files() {
        println!("Nothing to commit.");
        return ;
    }

    let user_res = read_to_string(Path::new(".tease/user"));
    if user_res.is_err() {
        println!("Set user before commiting.");
        return ;
    }

    let user = user_res.unwrap();

    let repo_tree = create_tree();
    write_trees(&repo_tree);

    let dt = UTC::now();
    let timestamp: i64 = dt.timestamp();

    let new_commit = Commit {
        tree: repo_tree.sha1_hash,
        parent: if is_merging() { format!("{} {}", read_head_commit(), index.incoming_merge) } else { read_head_commit() },
        author: format!("{} {}", user, timestamp),
        commiter: format!("{} {}", user, timestamp),
        message
    };

    let commit_content = format!("tree {}\nparent {}\nauthor {}\ncommiter {}\n\n{}", new_commit.tree, new_commit.parent, new_commit.author, new_commit.commiter, new_commit.message);

    let mut hasher = Sha1::new();
    hasher.update(commit_content.as_bytes());
    
    let binary_hash = hasher.finalize();
    let string_hash_vector: Vec<String> = binary_hash.iter().map(|n| format!("{:x?}", n)).collect();

    let commit_sha1 = string_hash_vector.join("");

    let write_res = compress_and_write_object(commit_content.as_bytes(), commit_sha1.to_string());
    if write_res.is_err() {
        println!("Couldn't commit.");
        return ;
    }
    update_head(commit_sha1.to_string()).unwrap();
    flush_index();

    println!("Commited {}", commit_sha1);
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