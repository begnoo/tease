mod commands;
mod index_structs;
mod utils;

use crate::commands::{create::create_repo, add::{add_from_path, delete_from_path}, read::read_object, reset::reset_index_row, commit::commit};
use commands::{status::status, branch::{create_branch, switch_to_branch}, diff::diff_file, merge::merge_file, command_enum::{Args, Commands}};
use clap::{Parser};

fn main() {
    let args = Args::parse();

    match &args.command {
        Some(Commands::Create { repo_name }) => {
            let deref_repo_name = repo_name.as_ref().map_or("tease_repo", |repo_name| repo_name);
            println!("tease cli trying to create {:?}...", deref_repo_name);            
            let result = create_repo(deref_repo_name.to_string());
            println!("{0}", result);
        }
        
        Some(Commands::Add { file_path, mode }) => {
            let deref_file_path = file_path.as_ref().unwrap().to_string();
            let deref_mode: String;

            if !mode.is_none() {
                deref_mode = mode.as_ref().unwrap().to_string()
            } else {
                deref_mode = "".to_string();
            }

            println!("tease cli trying to add {:?}...", deref_file_path);

            let result: String;
            if deref_mode == "delete" {
                result = delete_from_path(deref_file_path.to_string());
            } else {
                result = add_from_path(deref_file_path.to_string());
            }            

            println!("{0}", result);
        }
        
        Some(Commands::Commit { message }) =>  {
            println!("tease cli trying to commit {:?}...", message.join(" ").to_string());
            commit(message.join(" ").to_string());
        }

        Some(Commands::Read { object: object_path }) =>  {
            println!("tease cli trying to read {:?}...", object_path.to_string());
            let s = read_object(object_path);
            println!("{}", s);

        }
        
        Some(Commands::Reset { filename }) =>  {
            println!("tease cli trying to delete {:?}...", filename.to_string());
            reset_index_row(filename.to_string());
        }

        Some(Commands::Status) => {
            status();
        }

        Some(Commands::Branch { name, mode }) => { 
            let deref_mode: String;

            if !mode.is_none() {
                deref_mode = mode.as_ref().unwrap().to_string()
            } else {
                deref_mode = "".to_string();
            }

            if deref_mode == "create" {
                create_branch(name.to_string());
            } else {
                switch_to_branch(name.to_string());
            }
        }

        Some(Commands::Diff {blob_a, blob_b}) => {
            let diff_lines = diff_file(blob_a.to_string(), blob_b.to_string());
            for diff_line in diff_lines.iter() {
                println!("{}", diff_line);
            }
        }

        Some(Commands::Merge {blob_a, blob_b, blob_o}) => {
            merge_file(blob_a.to_string(), blob_b.to_string(), blob_o.to_string());
        }

        None => {
            println!("type tease --help for info.");
        }
    }

}
