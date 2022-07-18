use std::fs::metadata;
use std::fs::read_to_string;
use std::fs::File;
use std::io::Error;
use std::io::Write;
use flate2::Compression;
use flate2::write::ZlibEncoder;
use sha1::{Sha1, Digest};
use std::path::Path;

#[derive(Debug)]
pub struct ObjectInfo {
    object_type: String,
    size: u32,
    content: String,
    _filename: String
}

pub fn add_from_path(path: String) -> String {
    let file_md = metadata(path.to_string()).unwrap();
    
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

    Ok(format!("Added object [{}]", sha1_hash))
}

fn compress_and_write_object(object_data: &[u8], name: String) -> Result<(), Error> {
    let mut e = ZlibEncoder::new(Vec::new(), Compression::default());
    e.write_all(object_data)?;
    let compressed_bytes = e.finish().unwrap();

    let mut file = File::create(Path::new(".tease").join("objects").join(name))?;
    file.write_all(&compressed_bytes)?;

    Ok(())
}

fn get_sha1_hash(object_data: &[u8]) -> String {
    let mut hasher = Sha1::new();
    hasher.update(object_data);
    
    let binary_hash = hasher.finalize();
    let string_hash_vector: Vec<String> = binary_hash.iter().map(|n| format!("{:x?}", n)).collect();

    string_hash_vector.join("")
}
