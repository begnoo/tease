use serde::{Deserialize, Serialize};
use tease_common::read::blob_reader::{read_tree_from_commit, collect_from_tree};
use std::fs;
use std::path::Path;
use std::io::{Write, Error, ErrorKind};

use crate::utils::blob_writer::read_head_commit;


#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
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
    pub rows: Vec<IndexRow>,
    pub is_merging: bool,
    pub incoming_merge: String,
}

pub fn read_index() -> Index {
    let index_binary = fs::read(Path::new(".tease").join("index"))
        .expect("Couldn't read index file");
    let mut index: Index = bincode::deserialize(&index_binary).unwrap();
    index.rows.sort_by(|a, b| Ord::cmp(&a.file_name, &b.file_name));

    index
}

pub fn is_merging() -> bool{
    read_index().is_merging
}


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
    let same_name_row = index.rows.iter().position(|row| row.file_name == filename).unwrap();
    index.rows.remove(same_name_row);
    save_index(index)?;

    Ok(())
}


pub fn reset_index_row(filename: String) -> Result<(), Error> {
    let mut index = read_index();
    let mut found = false;

    let head_commit = read_head_commit();
    let root_tree = read_tree_from_commit(&".tease".to_string(), &head_commit);
    let objects = collect_from_tree(".tease".to_string(), root_tree.to_string());
    println!("{:?}", index.rows);

    for row in index.rows.iter_mut() {
        if row.staging == 1 {
            return Ok(());
        }

        if row.file_name == filename {
            let index_obj = objects.iter().find(|obj| obj.path == filename);
            let is_in_head = index_obj.is_some();
            if row.staging == 0 && !is_in_head {
                let rm_res = remove_index_row(filename.to_string());
                if rm_res.is_err() {
                    println!("Couldn't delete row from index.");
                    return Err(Error::new(ErrorKind::NotFound, "Couldn't find row with given path!"))
                }
                return Ok(());
            }
            if row.staging == 0 && is_in_head {
                row.data_change_date = 0;
                row.blob_hash = index_obj.unwrap().sha1.to_string();
            }
            
            row.staging = 1;
            found = true;
            break;
        }
    }
    if !found {
        return Err(Error::new(ErrorKind::NotFound, "Couldn't find row with given path!"));
    }
    let rows = index.rows.to_vec();
    save_index(index)?;
    println!("{:?}", rows);

    Ok(())
}

pub fn save_index(index: Index) -> Result<(), Error> {
    let mut index_file = fs::File::create(Path::new(".tease").join("index")).unwrap();
    let index_binary: Vec<u8> = bincode::serialize(&index).unwrap();
    index_file.write(&index_binary)?;

    Ok(())
}

pub fn flush_index() -> () {
    let mut index = read_index();
    index.rows.retain(|row| row.staging != 2);

    for row in index.rows.iter_mut() {
        row.staging = 1;
    }

    index.is_merging = false;
    save_index(index).expect("Couldn't flush index.");
}