use crate::{
    utils::{
        lines::get_content_from_sha1,
        blob_writer::{
            tease_file_exists,
            read_head_commit,
            create_index_file}
    },
    commands::read::read_object,
    index_structs::index::{
        read_index,
        Index,
        IndexRow,
        save_index
    }, merge_utils::merge_file::{merge_file, ResolveType},
};

use super::{add::add_file, goback::delete_all};
use std::{fs::{read_to_string, create_dir_all}, path::Path};

use tease_common::{
    write::bolb_writer::create_tease_file,
    read::blob_reader::trail_commit_history,
    read::blob_reader::read_tree_from_commit,
};


#[derive(Default, Debug)]
struct IndexObject {
    sha1: String,
    path: String,
}

pub fn merge_branch(branch_name: String) {
    let branch_head = format!("refs/heads/{}", branch_name.to_string());

    if !tease_file_exists(branch_head.to_string()) {
        println!("Branch named {}, does not exist", branch_name.to_string());
        return ;
    }

    let incoming_head_commit = read_to_string(Path::new(".tease").join(branch_head.to_string())).expect(&format!("Couldn't read {}", branch_head));
    let current_head_commit = read_head_commit();
    merge_commits(current_head_commit, incoming_head_commit);
}

pub fn merge_commits(current_commit: String, incoming_commit: String) {
    let common_commit = find_common_commit(current_commit, incoming_commit.to_string());

    if common_commit == "merged" {
        println!("Branch is already merged");
        return ;
    }

    let mut incoming_index = extract_index_from_commit(incoming_commit.to_string());
    let mut common_index = extract_index_from_commit(common_commit.to_string());
    let mut current_index = read_index();

    delete_all();
    create_index_file(Path::new(".tease").join("index").as_path());
    handle_index_diff(&mut current_index, &mut common_index, &mut incoming_index);
    update_index_for_merge(incoming_commit);
}

fn update_index_for_merge(incoming_head: String) {
    let mut index = read_index();
    index.is_merging = true;
    index.incoming_merge = incoming_head;
    save_index(index).expect("Couldn't update index for merge");
}

fn handle_index_diff(current_index: &mut Index, common_index: &mut Vec<IndexObject>, incoming_head: &mut Vec<IndexObject>) {
    let mut to_delete: Vec<IndexObject> = vec![];

    for common in common_index.iter() {

        let current_position = current_index.rows.iter().position(|current| current.file_name == common.path);
        let incoming_position = incoming_head.iter().position(|branch| branch.path == common.path);

        if current_position.is_some() && incoming_position.is_some() {
            let mut current_row = current_index.rows.get_mut(current_position.unwrap()).unwrap();
            let incoming_row = incoming_head.get_mut(incoming_position.unwrap()).unwrap();

            let chunks = merge_file(current_row.blob_hash.to_string(), incoming_row.sha1.to_string(), common.sha1.to_string());
            
            if chunks.iter().find(|chunk| matches!(chunk.resolve_type, ResolveType::Conflict)).is_some() {
                current_row.staging = 1;
            } else {
                current_row.staging = 0;
            }
            
            let content: Vec<String> = chunks.iter().map(|chunk| chunk.to_string()).collect();
            create_missing_folders_and_file(current_row.file_name.to_string(), content.join("\n"));
            add_file(current_row.file_name.to_string()).expect(&format!("Couldn't merge file {}", current_row.file_name.to_string()));
            
            incoming_head.remove(incoming_position.unwrap());
            current_index.rows.remove(current_position.unwrap());
        
        } else if current_position.is_some() && incoming_position.is_none() {
            to_delete.push(IndexObject { sha1: common.sha1.to_string(), path: common.path.to_string() });
            current_index.rows.remove(current_position.unwrap());
        }
    }

    handle_residual_branch_rows(incoming_head);
    handle_residual_current_rows(current_index, &common_index);
    handle_rows_to_remove(&to_delete);
}

fn handle_residual_current_rows(old_index: & Index, common_index: & Vec<IndexObject>) {

    let mut added: Vec<String> = vec![];

    for old_row in old_index.rows.iter() {
        if common_index.iter().find(|row| row.path == old_row.blob_hash).is_some() {
            continue;
        } 
        
        let lines = get_content_from_sha1(old_row.blob_hash.to_string());
        let content: Vec<String> = lines.iter().map(|line| line.to_string()).collect();
        create_missing_folders_and_file(old_row.file_name.to_string(), content.join(""));
        add_file(old_row.file_name.to_string()).expect(&format!("Couldn't merge file {}", old_row.file_name.to_string()));
        added.push(old_row.file_name.to_string());
    }
    
    let mut new_index = read_index();
    
    for branch_row in added.iter() {
        let new_row_position = new_index.rows.iter().position(|new_row| new_row.file_name == branch_row.to_string());
        if new_row_position.is_some() {
            let mut new_row = new_index.rows.get_mut(new_row_position.unwrap()).unwrap();
            new_row.staging = 2;
        }
    }
}

fn handle_residual_branch_rows(branch_index: & Vec<IndexObject>) {
    for branch_row in branch_index.iter() {
        let lines = get_content_from_sha1(branch_row.sha1.to_string());
        let content: Vec<String> = lines.iter().map(|line| line.to_string()).collect();
        create_missing_folders_and_file(branch_row.path.to_string(), content.join(""));
        add_file(branch_row.path.to_string()).expect(&format!("Couldn't merge file {}", branch_row.path.to_string()));
    }
    
    let mut new_index = read_index();
    
    for branch_row in branch_index.iter() {
        let new_row_position = new_index.rows.iter().position(|new_row| new_row.file_name == branch_row.path);
        if new_row_position.is_some() {
            let mut new_row = new_index.rows.get_mut(new_row_position.unwrap()).unwrap();
            new_row.staging = 2;
        }
    }
}

fn handle_rows_to_remove(to_remove: & Vec<IndexObject>) {
    let mut new_index = read_index();

    for row_to_remove in to_remove.iter() {
        let new_row = new_index.rows.iter().find(|new_row| new_row.file_name == row_to_remove.path);
        if new_row.is_none() {
            new_index.rows.push( IndexRow { file_name: row_to_remove.path.to_string(), blob_hash: row_to_remove.sha1.to_string(), staging: 2, ..Default::default()} );
        }
    }

    save_index(new_index).expect("Couldn't update rows to remove while merging");
}

fn create_missing_folders_and_file(filepath: String, content: String) {
    let parts: Vec<&str> = filepath.split("/").collect();
    if parts.len() > 1 {
        let folder_path = parts[0..parts.len() - 1].join("/");
        let path = Path::new(&folder_path);
        if !path.exists() {
            create_dir_all(path).expect(&format!("Couldn't create folder {}", path.to_str().unwrap()));
        }
    }
    create_tease_file(Path::new(&filepath), content);
}

fn extract_index_from_commit(commit: String) -> Vec<IndexObject> {
    let root_tree = read_tree_from_commit(&".tease".to_string(), &commit);
    let mut temp_index: Vec<IndexObject> = vec![];
    collect_from_branch(root_tree, "".to_string(), &mut temp_index);

    temp_index
}

fn is_already_merged(history: &Vec<String>, branch_head: String) -> bool {
    history.iter().find(|commit| commit.to_string() == branch_head).is_some()
}

fn find_common_commit(current: String, incoming: String) -> String {
    let mut current_history: Vec<String> = vec![current.to_string()];
    trail_commit_history(&".tease".to_string(), &current, &"#".to_string(), &mut current_history);

    if is_already_merged(&current_history, incoming.to_string()) {
        return "merged".to_string();
    }

    let mut incoming_history: Vec<String> = vec![incoming.to_string()];
    trail_commit_history(&".tease".to_string(), &incoming, &"#".to_string(), &mut incoming_history);

    for current_sha1 in current_history.iter() {
       for incoming_sha1 in incoming_history.iter() {
            if current_sha1 == incoming_sha1 {
                return current_sha1.to_string();
            }
       }   
    }

    "".to_string()
}

fn collect_from_branch(root_tree: String, prev_path: String, temp_index: & mut Vec<IndexObject>) {    
    let tree_content = read_object(&root_tree);
    let lines: Vec<&str> = tree_content.split("\n").collect();

    for line in lines {
        let parts: Vec<&str> = line.split(" ").collect();
        
        if parts[0] == "blob" {
            let new_file = if prev_path.is_empty() { parts[1].to_string() } else { vec![prev_path.to_string(), parts[1].to_string()].join("/") };
            temp_index.push(IndexObject { sha1: parts[2].to_string(), path: new_file });
        }

        if parts[0] == "tree" {
            let new_folder = if prev_path.is_empty() { parts[1].to_string() } else { vec![prev_path.to_string(), parts[1].to_string()].join("/") };
            collect_from_branch(parts[2].to_string(), new_folder.to_string(), temp_index);
        }
    }
}

