use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use std::io::{Write, Error};


#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct IndexRow {
    pub data_change_date: u64,
    pub meta_change_date: u64,
    pub file_size: u64,
    pub file_name: String,
    pub blob_hash: String,
    pub staging: u64,
    pub user_id: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Index {
    pub rows: Vec<IndexRow>
}

pub fn read_index() -> Index {
    let index_binary = fs::read(Path::new(".tease").join("index"))
        .expect("Coundn't read index file");
    let index: Index = bincode::deserialize(&index_binary).unwrap();
    
    index
}

pub fn save_index(index: Index) -> Result<(), Error> {
    let mut index_file = fs::OpenOptions::new()
                                    .write(true)
                                    .open(Path::new(".tease").join("index"))
                                    .unwrap();
    
    let index_binary: Vec<u8> = bincode::serialize(&index).unwrap();
    index_file.write(&index_binary)?;

    Ok(())
}