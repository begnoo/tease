mod commands;
mod index_structs;
mod utils;
mod remote_req;
mod merge_utils;

use std::path::Path;

use crate::{commands::{
    create::create_repo,
    add::{add_from_path, delete_from_path},
    read::read_object,
    commit::commit, reset::reset},
    index_structs::index::is_merging
};
use commands::{
    status::status,
    branch::{create_branch, switch_to_branch, create_from_remote},
    diff::diff_file,
    merge::merge_branch, 
    command_enum::{Args, Commands}, 
    set_origin::set_origin, set_user::set_user, push::push, goback::go_back, pull::pull, clone::clone, init::init
};
use clap::Parser;
use merge_utils::merge_file::merge_file;
use tease_common::read::blob_reader::trail_commits_all;
use utils::blob_writer::{has_added_files, read_head_commit};

// TODO: packfile
// TODO: skinuti lock za branch pri create modu i preneti trenutne izmene i dodate fajlove
// TODO: dodaj info o razlici kod commitova (+) (-)
fn main() {
    let args = Args::parse();

    match &args.command {
        Some(Commands::Create { repo_name }) => {
            let deref_repo_name = repo_name.as_ref().map_or("tease_repo", |repo_name| repo_name);
            println!("tease cli trying to create {:?}.", deref_repo_name);            
            let _result = create_repo(deref_repo_name.to_string());
        }
        
        Some(Commands::Add { file_path }) => {
            if !in_working_tree() {
                println!("Not in working tree.");
                return ;
            }

            println!("tease cli trying to add {:?}.", file_path.to_string());
            let res = add_from_path(file_path.to_string());
            println!("{}", res);  
        }

        Some(Commands::Rm { file_path }) =>  {
            if !in_working_tree() {
                println!("Not in working tree.");
                return ;
            }

            println!("tease cli trying to delete {:?}.", file_path.to_string());
            let res = delete_from_path(file_path.to_string());
            println!("{}", res);
        }
        
        Some(Commands::Commit { message }) =>  {
            if !in_working_tree() {
                println!("Not in working tree.");
                return ;
            }

            println!("tease cli trying to commit {:?}.", message.join(" ").to_string());
            commit(message.join(" ").to_string());
        }

        Some(Commands::Read { object: object_path }) =>  {
            if !in_working_tree() {
                println!("Not in working tree.");
                return ;
            }
            
            println!("tease cli trying to read {:?}.", object_path.to_string());
            let s = read_object(object_path);
            println!("{}", s);
        }
        
        Some(Commands::Reset { filename }) =>  {
            if !in_working_tree() {
                println!("Not in working tree.");
                return ;
            }
            
            println!("tease cli reset index row {:?}.", filename.to_string());
            reset(filename.to_string());
        }

        Some(Commands::Status) => {
            if !in_working_tree() {
                println!("Not in working tree.");
                return ;
            }

            status();
        }

        Some(Commands::Branch { name, mode }) => {
            if !in_working_tree() {
                println!("Not in working tree.");
                return ;
            }

            if is_merging() {
                println!("Please confirm merge before branching.");
                return ;
            }

            if has_added_files() {
                println!("Please commit your changes before switching branches.");
                return ;
            }
            
            if mode.is_some() {
                let m = mode.to_owned().unwrap();
                match m.as_str() {
                    "c" => create_branch(name.to_string()),
                    "rc" => create_from_remote(name.to_string()),
                    _ => println!("Unsuported mode.") 
                }
            } else {
                switch_to_branch(name.to_string());
            }
        }

        Some(Commands::Diff {blob_a, blob_b}) => {
            if !in_working_tree() {
                println!("Not in working tree.");
                return ;
            }

            let diff_lines = diff_file(blob_a.to_string(), blob_b.to_string());
            for diff_line in diff_lines.iter() {
                println!("{}", diff_line);
            }
        }

        Some(Commands::MergeFile {blob_a, blob_b, blob_o}) => {
            if !in_working_tree() {
                println!("Not in working tree.");
                return ;
            }
            
            let chunks = merge_file(blob_a.to_string(), blob_b.to_string(), blob_o.to_string());
            for chunk in chunks.iter() {
                print!("{}", chunk);
            }
        }

        Some(Commands::Merge {branch}) => {
            if !in_working_tree() {
                println!("Not in working tree.");
                return ;
            }
            
            if is_merging() {
                println!("Please confirm merge before branching.");
                return ;
            }
            merge_branch(branch.to_string());
        }

        Some(Commands::SetOrigin {origin}) => {
            if !in_working_tree() {
                println!("Not in working tree.");
                return ;
            }
            
            set_origin(origin.to_string());
        },

        Some(Commands::SetUser {email}) => {
            if !in_working_tree() {
                println!("Not in working tree.");
                return ;
            }
            
            set_user(email.to_string());
        }

        Some(Commands::Push) => {
            if !in_working_tree() {
                println!("Not in working tree.");
                return ;
            }

            push();   
        }

        Some(Commands::Pull) => {
            if !in_working_tree() {
                println!("Not in working tree.");
                return ;
            }
            
            pull();   
        }

        Some(Commands::Clone { origin }) => {           
            clone(origin.to_string());   
        }

        Some(Commands::GoBack { sha }) => {
            if !in_working_tree() {
                println!("Not in working tree.");
                return ;
            }

            go_back(sha.to_string());   
        }

        Some(Commands::Init { name }) => {
            init(name.to_string());   
        }

        Some(Commands::Log {}) => {
            let head = read_head_commit();
            let mut log_trail = trail_commits_all(".tease".to_string(), head);
            log_trail.sort_by(|a, b| b.date.cmp(&a.date));
            log_trail.dedup_by(|a, b| a.sha1 == b.sha1);

            for commit in log_trail.iter() {
                println!("{}\n", commit);
            }
        }

        None => {
            println!("type tease --help for info.");
        }
    }

}


fn in_working_tree() -> bool {
    let path = Path::new(".tease");
    if path.exists() && path.is_dir() {
        return true;
    }
    
    false
}