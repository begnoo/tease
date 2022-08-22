use std::fs::File;
use std::fs::Metadata;
use std::fs::create_dir;
use std::fs::metadata;
use std::fs::read_to_string;

use std::io::Error;
use std::io::Write;

use std::path::Path;

use flate2::Compression;
use flate2::write::ZlibEncoder;

use std::time::{UNIX_EPOCH};

use crate::index_structs::index::Index;
use crate::index_structs::index::read_index;

use super::glob::get_all_repo_paths;


pub fn compress_and_write_object(object_data: &[u8], name: String) -> Result<(), Error> {
    let mut e = ZlibEncoder::new(Vec::new(), Compression::default());
    e.write_all(object_data)?;
    let compressed_bytes = e.finish().unwrap();

    let mut file = File::create(Path::new(".tease").join("objects").join(name))?;
    file.write_all(&compressed_bytes)?;

    Ok(())
}

pub fn read_file_md(filename: String) -> Metadata {
    let file = File::open(Path::new(&filename))
        .expect("Couldn't read added file");

    file.metadata().expect("Couldn't get file metadata.")
}

pub fn get_metadata_change(metadata: &Metadata) -> u64 {
    metadata.modified()
        .unwrap()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

pub fn create_index_file(path: &Path) -> () {
    let index = Index {
        rows: Vec::new(),
        is_merging: false,
        incoming_merge: "".to_string(),
    };
    let mut file = File::create(path).expect(&format!("Couldn't create file {:?}", path.to_str().unwrap().to_string()));
    let encoded_index: Vec<u8> = bincode::serialize(&index).expect("Couldn't serialize index");
    file.write(&encoded_index).expect("Couldn't write index binary");
}

pub fn create_tease_folder(path: &Path) -> () {
    if path.exists() {
        return ;
    }

    create_dir(path).expect(&format!("Couldn't create folder {:?}", path.to_str().unwrap().to_string()));
}

pub fn update_current_branch(branch_head: String) -> Result<(), Error> {
    let mut file = File::create(Path::new(".tease").join("HEAD"))
                                .expect(&format!("Couldn't read HEAD file"));

    write!(file, "{}", branch_head)
}

pub fn get_current_branch() -> String {
    read_to_string(Path::new(".tease").join("HEAD"))
        .expect("Something went wrong reading the HEAD file")
}

pub fn read_head_commit() -> String {
    let current_ref_head = get_current_branch();
    let head_commit = read_to_string(Path::new(".tease").join(current_ref_head.to_string()))
        .expect(&format!("Couldn't read {}", current_ref_head));
    
    head_commit
}


pub fn read_origin_head_commit() -> String {
    let current_ref_head = get_current_branch();
    let head_commit = read_to_string(Path::new(".tease")
        .join(format!("{}-origin", current_ref_head.to_string())));
    
    if head_commit.is_err() {
        return "".to_string();
    }

    head_commit.unwrap()
}

pub fn update_head(commit_sha1: String) -> Result<(), Error>{
    let current_ref_head = get_current_branch();
    let mut file = File::create(Path::new(".tease").join(current_ref_head.to_string()))
                                .expect(&format!("Couldn't read {}", current_ref_head));
    write!(file, "{}", commit_sha1)
}

pub fn update_origin_head(commit_sha1: String) -> Result<(), Error>{
    let current_ref_head = get_current_branch();
    let mut file = File::create(Path::new(".tease")
                                                .join(format!("{}-origin", current_ref_head.to_string())))
                                .expect(&format!("Couldn't read {}", current_ref_head));
    write!(file, "{}", commit_sha1)
}

pub fn tease_file_exists(path: String) -> bool {
    let md = metadata(Path::new(".tease").join(path));

    md.is_ok()
}

pub fn has_added_files() -> bool {
    let index = read_index();

    index.rows.iter().any(|row| row.staging == 0 || row.staging == 2)
}

pub fn has_untracked_files() -> bool {
    let index = read_index();
    let entries: Vec<String> = get_all_repo_paths().iter().filter(|entry| {
        let file_md = metadata(entry.to_string());
        if file_md.is_err() {
            return false;
        }

        if file_md.unwrap().is_dir() {
            return false;
        }

        return true;
    })
    .map(|entry| entry.to_string())
    .collect();

    for entry_data in entries {

        let row = index.rows.iter().find(|row| row.file_name == entry_data);

        if row.is_none() {
            return true;
        }
    }

    false
}

pub fn get_origin() -> String {
    read_to_string(Path::new(".tease/origin")).expect(&format!("Couldn't read origin"))
}