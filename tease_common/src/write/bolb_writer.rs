use std::{path::Path, fs::File, io::Write};

pub fn create_tease_file(path: &Path, message: String) -> () {
    let file_path = path.to_str()
                        .unwrap()
                        .to_string();
                        
    let mut file = File::create(path)
        .expect(&format!("Couldn't create file {:?}", file_path.to_string()));
    
    file.write_all(message.as_bytes())
        .expect(&format!("Couldn't write to file {:?}", file_path));
}