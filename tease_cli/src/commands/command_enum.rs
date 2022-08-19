use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// Direct command such as create, clone, commit, push 
    #[clap(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    
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
    },
    
    //merge files
    MergeFile {
        blob_a: String,
        blob_b: String,
        blob_o: String,
    },

    Merge {
        branch: String,
    },

    SetOrigin {
        origin: String,
    },

    SetUser {
        email: String,
    },

    Push,

    Pull,

    Clone {
        origin: String,
    },

    GoBack {
        sha: String,
    }
}