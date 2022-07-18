use flate2::read::ZlibDecoder;
use std::fs::File;
use std::io::Read;
use std::path::Path;


pub fn read_object(object_name: &String) -> () {
    let object_file = File::open(&Path::new(".tease").join("objects").join(object_name))
        .expect(&format!("Coundn't read object {}", object_name));
    let mut decoder = ZlibDecoder::new(object_file);
    let mut s = String::new();
    decoder.read_to_string(&mut s).unwrap();
    println!("{}", s);
}