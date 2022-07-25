use glob::glob;

use crate::index_structs::index::Index;
use crate::index_structs::index::IndexRow;
use crate::index_structs::index::read_index;
use crate::utils::blob_writer::get_current_branch;
use crate::utils::blob_writer::get_metadata_change;
use crate::utils::blob_writer::read_file_md;

use colored::Colorize;

use std::os::windows::prelude::MetadataExt;

pub fn status() {
    let path_entries = glob("./**/*").expect("Failed to read glob pattern");
    let entries = path_entries.into_iter()
                                            .map(|entry| entry.unwrap()
                                                                                            .to_str()
                                                                                            .unwrap()
                                                                                            .to_string()
                                                                                            .replace("\\", "/"))
                                                                                            .collect();

    let index = read_index();
    
    let mut staged_vec: Vec<String> = vec![];
    let mut unstaged_vec: Vec<String> = vec![];
    let mut untracked_vec: Vec<String> = vec![];
    let mut deleted_vec: Vec<String> = detect_deleted(&index, &entries);
    let to_be_deleted_vec: Vec<String> = detect_to_be_deleted(&index, &deleted_vec);
    deleted_vec.retain(|row| !to_be_deleted_vec.contains(row));

    for entry_data in entries {

        let row = index.rows.iter().find(|row| row.file_name == entry_data);

        if row.is_none() {
            untracked_vec.push(entry_data);
            continue;
        }

        let unwraped_row = row.unwrap();

        if unwraped_row.staging == 0 {
            staged_vec.push(entry_data);
            continue;
        }

        if unwraped_row.staging == 1 {
            let current_file_md = read_file_md(entry_data.to_string());
            
            if unwraped_row.data_change_date != current_file_md.last_write_time() || unwraped_row.meta_change_date != get_metadata_change(&current_file_md) {
                unstaged_vec.push(entry_data);
            }
        }
    }

    format_data(&staged_vec, &unstaged_vec, &untracked_vec, &to_be_deleted_vec, &deleted_vec);
}

fn detect_deleted(index: &Index, entries: &Vec<String>) -> Vec<String> {
    index.rows.iter()
                .filter(|row| !entries.contains(&row.file_name))
                .collect::<Vec<&IndexRow>>()
                .iter()
                .map(|row| row.file_name.to_string())
                .collect::<Vec<String>>()
}

fn detect_to_be_deleted(index: &Index, deleted_vec: &Vec<String>) -> Vec<String> {
    index.rows.iter()
                .filter(|row| deleted_vec.contains(&row.file_name) && row.staging == 2)
                .collect::<Vec<&IndexRow>>()
                .iter()
                .map(|row| row.file_name.to_string())
                .collect::<Vec<String>>()
}

fn format_data(
        staged_vec: &Vec<String>,
        unstaged_vec: &Vec<String>,
        untracked_vec: &Vec<String>,
        to_be_deleted_vec: &Vec<String>,
        deleted_vec: &Vec<String>
    ) -> () {

    let current_branch_ref = get_current_branch();
    println!("You are on the branch: {:?}", current_branch_ref);

    let staged = staged_vec.join("\n\t");
    let unstaged = unstaged_vec.join("\n\t");
    let untracked = untracked_vec.join("\n\t");
    let to_be_deleted = to_be_deleted_vec.join("\n\t");
    let deleted = deleted_vec.join("\n\t");

    let staging = format!("Staging files:\n\t{}\n\t{}",
                                        format!("{}", staged).green(), 
                                        format!("{}", unstaged).red());
    let deletion = format!("Deleted files:\n\t{}\n\t{}", 
                                        format!("{}", to_be_deleted).green(),
                                        format!("{}", deleted).red());
    let tracking = format!("Untracked files:\n\t{}", format!("{}", untracked).yellow());

    println!("{}\n{}\n{}", staging, deletion, tracking);
    
}