mod commands;
mod index_structs;

use crate::commands::{create::create_repo, add::add_from_path, read::read_object, reset::reset_index_row};
use clap::{Parser, Subcommand};

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
        file_path: Option<String>
    },
    
    /// Commit added file changes to repo
    Commit { 
        /// Commit message for added changes
        message: Option<String> 
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
        
        Some(Commands::Add { file_path }) => {
            let deref_file_path = file_path.as_ref().map_or(".", |file_path| file_path);
            println!("tease cli trying to add {:?}...", deref_file_path);
            let result = add_from_path(deref_file_path.to_string());
            println!("{0}", result);
        }
        
        Some(Commands::Commit { message }) =>  {
            let commit_message =  message.as_ref().map_or("default", |message| message);
            println!("tease cli trying to commit {:?}...", commit_message);
        }

        Some(Commands::Read { object: object_path }) =>  {
            println!("tease cli trying to read {:?}...", object_path.to_string());
            read_object(object_path)
        }

        
        Some(Commands::Reset { filename }) =>  {
            println!("tease cli trying to delete {:?}...", filename.to_string());
            reset_index_row(filename.to_string());
        }

        None => {
            println!("type tease --help for info.");
        }
    }

}
