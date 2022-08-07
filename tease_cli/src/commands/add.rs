use std::fs::metadata;
use std::fs::read_to_string;
use std::fs::read_dir;
use std::fs::File;

use std::io::Error;

use std::os::windows::prelude::MetadataExt;
use std::path::Path;

use std::time::{UNIX_EPOCH};

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
    let file_md = metadata(path.to_string()).expect("Couldn't find specified path.");
    
    if file_md.is_dir() {
        return handle_dir(path.to_string()); 
    }

    add_file(path.to_string()).unwrap()
}

fn handle_dir(dir_path: String) -> String {
    let dir = Path::new(&dir_path);
    for entry in read_dir(dir).expect(&format!("Couldn't read dir {}", dir_path)) {
        let file_entry = entry.unwrap();
        let path = file_entry.path();
        let path_str = path.to_str().unwrap().to_string().replace("\\", "/");
        add_from_path(path_str.to_string());
    }

    format!("Added folder {}", dir_path)
}

pub fn add_file(filename: String) -> Result<String, Error> {

    let content = read_to_string(filename.to_string())
        .expect(&format!("Something went wrong reading the file {:?}", filename.to_string()));

    let object_info = ObjectInfo {
        object_type: "blob".to_string(),
        size: content.len() as u32,
        _filename: filename.to_string(),
        content
    };

    let object_data = format!("{} {}\0{}", object_info.object_type, object_info.size, object_info.content);

    let sha1_hash = get_sha1_hash(object_data.as_bytes());
    compress_and_write_object(object_data.as_bytes(), sha1_hash.to_string())
        .expect(&format!("Couldn't add object for file {:?}", filename.to_string()));
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
    let file = File::open(Path::new(filename))
        .expect("Couldn't read added file");

    let metadata = file.metadata().unwrap();
    let meta_change_date = metadata.modified()
                                        .unwrap()
                                        .duration_since(UNIX_EPOCH)
                                        .unwrap()
                                        .as_secs();

    let index_row = IndexRow {
        // TODO: dodati i linux i windows metode
        data_change_date: metadata.last_write_time(),
        meta_change_date,
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
        
        return format!("Deleted file {}", path);
    }

    format!("File not deleted localy {}", path)
}