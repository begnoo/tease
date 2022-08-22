mod commands;
mod index_structs;
mod utils;
mod remote_req;
mod merge_utils;

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
use utils::blob_writer::has_added_files;

// TODO: packfile
// TODO: rekurzivne funkcije -> iterativne (collect_objects_from_tree -> collect_from_tree)
// TODO: skinuti lock za branch pri create modu i preneti trenutne izmene i dodate fajlove
// TODO: sredi log da prikazuje commitove po redosledu a ne po roditeljima (mozda bitno samo za front)
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
            println!("tease cli trying to add {:?}.", file_path.to_string());
            let res = add_from_path(file_path.to_string());
            println!("{}", res);  
        }

        Some(Commands::Rm { file_path }) =>  {
            println!("tease cli trying to delete {:?}.", file_path.to_string());
            let res = delete_from_path(file_path.to_string());
            println!("{}", res);
        }
        
        Some(Commands::Commit { message }) =>  {
            println!("tease cli trying to commit {:?}.", message.join(" ").to_string());
            commit(message.join(" ").to_string());
        }

        Some(Commands::Read { object: object_path }) =>  {
            println!("tease cli trying to read {:?}.", object_path.to_string());
            let s = read_object(object_path);
            println!("{}", s);
        }
        
        Some(Commands::Reset { filename }) =>  {
            println!("tease cli reset index row {:?}.", filename.to_string());
            reset(filename.to_string());
        }

        Some(Commands::Status) => {
            status();
        }

        Some(Commands::Branch { name, mode }) => {

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
            let diff_lines = diff_file(blob_a.to_string(), blob_b.to_string());
            for diff_line in diff_lines.iter() {
                println!("{}", diff_line);
            }
        }

        Some(Commands::MergeFile {blob_a, blob_b, blob_o}) => {
            let chunks = merge_file(blob_a.to_string(), blob_b.to_string(), blob_o.to_string());
            for chunk in chunks.iter() {
                print!("{}", chunk);
            }
        }

        Some(Commands::Merge {branch}) => {
            if is_merging() {
                println!("Please confirm merge before branching.");
                return ;
            }
            merge_branch(branch.to_string());
        }

        Some(Commands::SetOrigin {origin}) => {
            set_origin(origin.to_string());
        },

        Some(Commands::SetUser {email}) => {
            set_user(email.to_string());
        }

        Some(Commands::Push) => {
            push();   
        }

        Some(Commands::Pull) => {
            pull();   
        }

        Some(Commands::Clone { origin }) => {
            clone(origin.to_string());   
        }

        Some(Commands::GoBack { sha }) => {
            go_back(sha.to_string());   
        }

        Some(Commands::Init { name }) => {
            init(name.to_string());   
        }

        None => {
            println!("type tease --help for info.");
        }
    }

}
