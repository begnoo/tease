use std::fs::File;

use std::fs::Metadata;
use std::io::Error;
use std::io::Write;

use std::path::Path;

use flate2::Compression;
use flate2::write::ZlibEncoder;

use std::time::{UNIX_EPOCH};



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

// pub fn row_index_from_metadata(metadata: Metadata) -> IndexRow {


//     IndexRow {
//         // TODO: dodati i linux i windows metode
//         data_change_date: metadata.last_write_time(),
//         meta_change_date,
//         file_size: metadata,
//         file_name: filename.to_string(),
//         blob_hash: sha1_hash.to_string(),
//         staging: 0,
//         user_id: "Nzm za sada".to_string(),
//     }
// }