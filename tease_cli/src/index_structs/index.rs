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
        .expect("Couldn't read index file");
    let index: Index = bincode::deserialize(&index_binary).unwrap();
    
    index
}


// TODO : tree
pub fn add_index_row(index_row: IndexRow) -> Result<(), Error> {
    let mut index = read_index();

    let existing_row = index.rows.iter().find(|row| row.blob_hash == index_row.blob_hash && row.file_name == index_row.file_name);
    if !existing_row.is_none() {
        return Ok(());    
    }

    let same_name_row = index.rows.iter().position(|row| row.blob_hash != index_row.blob_hash && row.file_name == index_row.file_name);
    if !same_name_row.is_none() {
        let same_name_row_index = same_name_row.unwrap();
        let _old_value = std::mem::replace(&mut index.rows[same_name_row_index], index_row);
        save_index(index).expect("Couldn't update index value");

        return Ok(());
    }

    index.rows.push(index_row);
    save_index(index).expect("Couldn't update index value");

    Ok(())
}

pub fn remove_index_row(filename: String) -> Result<(), Error> {
    let mut index = read_index();
    let same_name_row = index.rows.iter().position(|row| row.file_name == filename);
    index.rows.remove(same_name_row.expect("No such file in index."));
    save_index(index)?;

    Ok(())
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