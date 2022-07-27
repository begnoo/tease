mod commands;
mod index_structs;
mod utils;

use crate::commands::{create::create_repo, add::{add_from_path, delete_from_path}, read::read_object, reset::reset_index_row, commit::commit};
use clap::{Parser, Subcommand};
use commands::{status::status, branch::{create_branch, switch_to_branch}, diff::diff};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Direct command such as create, clone, commit, push 
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    
    /// Create new tease repo
    Create {
        /// New repo name, if left empty it will be tease_repo.
        repo_name: Option<String> 
    },

    /// Add file changes to commit 
    Add {
        /// Path to file or folder which you want to add to the commit, if left empty all changes will be added.
        file_path: Option<String>,

        #[clap(short, long, value_parser)]
        mode: Option<String>,
    },
    
    /// Commit added file changes to repo
    Commit { 
        /// Commit message for added changes
        message: Vec<String> 
    },

    /// read object
    Read { 
        // object name
        object: String 
    },

    /// delete file from index, or return to older version
    Reset { 
        // file name
        filename: String 
    },

    // show current status of tracked and untracked files
    Status,

    // create or change branch
    Branch {
        name: String,
        #[clap(short, long, value_parser)]
        mode: Option<String>,
    },

    // show difference between two files
    Diff {
        blob_a: String,
        blob_b: String
    }
}

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
            diff(blob_a.to_string(), blob_b.to_string());
        }

        None => {
            println!("type tease --help for info.");
        }
    }

}
