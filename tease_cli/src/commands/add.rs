use std::fs::metadata;
use std::fs::read_to_string;
use std::fs::File;

use std::io::Error;
use std::io::Write;

use std::os::windows::prelude::MetadataExt;
use std::path::Path;

use std::time::{UNIX_EPOCH};

use flate2::Compression;
use flate2::write::ZlibEncoder;

use sha1::{Sha1, Digest};

use crate::index_structs::index::IndexRow;
use crate::index_structs::index::add_index_row;
use crate::index_structs::index::read_index;


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
        return "That's a directory".to_string(); 
    }

    add_file(path.to_string()).unwrap()
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
    print!("{}", object_data);

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


pub fn compress_and_write_object(object_data: &[u8], name: String) -> Result<(), Error> {
    let mut e = ZlibEncoder::new(Vec::new(), Compression::default());
    e.write_all(object_data)?;
    let compressed_bytes = e.finish().unwrap();

    let mut file = File::create(Path::new(".tease").join("objects").join(name))?;
    file.write_all(&compressed_bytes)?;

    Ok(())
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

    let new_index = read_index();
    println!("{:?}", new_index);
}
