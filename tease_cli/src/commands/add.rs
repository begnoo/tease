use std::fs::metadata;
use std::fs::read_to_string;
use std::fs::read_dir;
use std::fs::File;

use std::fs::remove_file;
use std::io::Error;

use std::os::unix::prelude::MetadataExt;
use std::path::Path;

use crate::index_structs::index::IndexRow;
use crate::index_structs::index::add_index_row;
use crate::index_structs::index::read_index;
use crate::index_structs::index::save_index;
use crate::utils::blob_writer::compress_and_write_object;

use sha1::{Sha1, Digest};

#[derive(Debug)]
pub struct ObjectInfo {
    object_type: String,
    size: u32,
    content: String,
    _filename: String
}

pub fn add_from_path(path: String) -> String {
    let file_md = metadata(path.to_string());
    
    if file_md.is_err() {
        return format!("Couldn't find file {:?}", path);
    }   

    if file_md.unwrap().is_dir() {
        return handle_dir(path.to_string()); 
    }

    let res = add_file(path.to_string());

    if res.is_err() {
        println!("Couldn't add file {:?}", path);
    }

    res.unwrap()
}

fn handle_dir(dir_path_str: String) -> String {
    let dir_path = Path::new(&dir_path_str);
    let dir = read_dir(dir_path).expect(&format!("Couldn't read dir {}", dir_path_str));
    for entry in dir {
        let file_entry = entry.unwrap();
        if file_entry.file_name().to_str().unwrap().contains(".tease") {
            continue;
        }
        let path = file_entry.path();
        let mut path_str = path.to_str().unwrap().to_string().replace("\\", "/");
        if path_str.starts_with("./") {
            path_str =  path_str.replace("./", "");
        }
        add_from_path(path_str.to_string());
    }

    format!("Added folder {}", dir_path_str)
}

pub fn add_file(filename: String) -> Result<String, Error> {

    let content_res = read_to_string(filename.to_string());
    if content_res.is_err() {
        return content_res;   
    }
    let content = content_res.unwrap();

    let object_info = ObjectInfo {
        object_type: "blob".to_string(),
        size: content.len() as u32,
        _filename: filename.to_string(),
        content
    };

    let object_data = format!("{} {}\0{}", object_info.object_type, object_info.size, object_info.content);

    let sha1_hash = get_sha1_hash(object_data.as_bytes());
    let write_res = compress_and_write_object(object_data.as_bytes(), sha1_hash.to_string());
    if write_res.is_err() {
        return Err(write_res.err().unwrap());   
    }
    
    add_to_index(&sha1_hash, &filename, object_info.size);

    Ok(format!("Added object [{}]", sha1_hash))
}

fn get_sha1_hash(object_data: &[u8]) -> String {
    let mut hasher = Sha1::new();
    hasher.update(object_data);
    
    let binary_hash = hasher.finalize();
    let string_hash_vector: Vec<String> = binary_hash.iter().map(|n| format!("{:x?}", n)).collect();

    string_hash_vector.join("")
}

fn add_to_index(sha1_hash: &String, filename: &String, file_size: u32) {
    let file_res = File::open(Path::new(filename));
    if file_res.is_err() {
        println!("Couldn't read added file");
        return ;
    }    

    let metadata = file_res.unwrap().metadata().unwrap();
    let index_row = IndexRow {
        // TODO: dodati i linux i windows metode
        data_change_date: metadata.ctime() as u64,
        meta_change_date: metadata.mtime() as u64,
        file_size: file_size as u64,
        file_name: filename.to_string(),
        blob_hash: sha1_hash.to_string(),
        staging: 0,
        user_id: "Nzm za sada".to_string(),
    };

    add_index_row(index_row).unwrap();
}


pub fn delete_from_path(path: String) -> String {
    let mut index = read_index();

    let mut found = false;
    
    for row in index.rows.iter_mut() {
        if row.file_name == path {
            row.staging = 2;
            found = true;
            break;
        }
    }

    if found {
        save_index(index).expect("Couldn't save index");
        
        let path_copy = path.to_string();
        let file_path = Path::new(&path_copy);
        if file_path.exists() {
            let rm_res = remove_file(file_path);
            if rm_res.is_err() {
                println!("Couldn't delete file {} from file system.", path);
            }
        }
        return format!("Deleted file {}", path);
    }

    format!("File {:?} couldn't be removed from index.", path)
}