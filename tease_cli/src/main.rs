mod commands;
mod index_structs;
mod utils;

use crate::{commands::{create::create_repo, add::{add_from_path, delete_from_path}, read::read_object, reset::reset_index_row, commit::commit}, index_structs::index::is_merging};
use commands::{status::status, branch::{create_branch, switch_to_branch}, diff::diff_file, merge::{merge_file, merge}, command_enum::{Args, Commands}};
use clap::{Parser};
use utils::blob_writer::has_added_files;

// 08.08 ako stignes (vrv neces)
// TODO: packfile, author*, commiter*, |.| dodavanje na add*

// Web
// povezi sa user servisom (email, username, login)
// povezi sa repo serverom (push, pull, clone)

// Malo radi web-a pa onda (uradi bar user i repo service)
// TODO: sredi log da prikazuje commitove po redosledu a ne po roditeljima (mozda bitno samo za front)
// TODO: dodaj info o razlici kod commitova (+) (-)

// Ako ne stignes do 20-25of -> ubi se
fn main() {
    let args = Args::parse();

    match &args.command {
        Some(Commands::Create { repo_name }) => {
            let deref_repo_name = repo_name.as_ref().map_or("tease_repo", |repo_name| repo_name);
            println!("tease cli trying to create {:?}...", deref_repo_name);            
            let _result = create_repo(deref_repo_name.to_string());
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

            let _result: String;
            if deref_mode == "delete" {
                _result = delete_from_path(deref_file_path.to_string());
            } else {
                _result = add_from_path(deref_file_path.to_string());
            }            
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

            if is_merging() {
                println!("Please confirm merge before branching.");
                return ;
            }

            if has_added_files() {
                println!("Please commit your changes before switching branches.");
                return ;
            }
            
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

        Some(Commands::MergeFile {blob_a, blob_b, blob_o}) => {
            let chunks = merge_file(blob_a.to_string(), blob_b.to_string(), blob_o.to_string());
            for chunk in chunks.iter() {
                println!("{}", chunk);
            }
        }

        Some(Commands::Merge {branch}) => {
            if is_merging() {
                println!("Please confirm merge before branching.");
                return ;
            }

            merge(branch.to_string());
        }

        None => {
            println!("type tease --help for info.");
        }
    }

}
