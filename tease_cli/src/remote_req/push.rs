use std::{io::{Write, Seek, Read}, path::Path, fs::{File, read_to_string}};

use reqwest::{Client, Body};
use tokio_util::codec::{FramedRead, BytesCodec};
use zip::write::FileOptions;

use crate::utils::blob_writer::get_current_branch;

use super::can_push::CanPushResponse;

use tokio::fs::File as TokioFile;

pub fn setup_post (cp: CanPushResponse) -> bool {
    let mut objects: Vec<String> = cp.diff.iter()
                         .map( |obj| format!(".tease/objects/{}", obj))
                         .collect();

    objects.push(format!(".tease/{}", get_current_branch()));

    let temp_zip = File::create(".tease/temp_zip").unwrap();
    let res = zip_dir(objects, temp_zip, zip::CompressionMethod::Stored);
    if res.is_err() {
        println!("Couldn't archive objects to send...");
        return false;
    }
    
    true
}

#[tokio::main]
pub async fn post_push () -> Result<(), Box<dyn std::error::Error>> {
    let file = TokioFile::open(".tease/temp_zip").await?;

    let client = Client::new();
    let res = client.post(get_origin())
        .body(file_to_body(file))
        .send()
        .await?;
    println!("{:?}", res);

    Ok(())
}

fn file_to_body(file: TokioFile) -> Body {
    let stream = FramedRead::new(file, BytesCodec::new());
    let body = Body::wrap_stream(stream);
    body
}


fn zip_dir<T>(
    entries: Vec<String>,
    writer: T,
    method: zip::CompressionMethod,
) -> zip::result::ZipResult<()>
where
    T: Write + Seek,
{
    let mut zip = zip::ZipWriter::new(writer);
    let options = FileOptions::default()
        .compression_method(method)
        .unix_permissions(0o755);

    let mut buffer = Vec::new();
    for entry in entries.iter() {
        let path = Path::new(entry);
        let name = path.strip_prefix(Path::new(".tease")).unwrap();

        // Write file or directory explicitly
        // Some unzip tools unzip files with directory paths correctly, some do not!
        if path.is_file() {
            println!("adding file {:?} as {:?} ...", path, name);
            #[allow(deprecated)]
            zip.start_file_from_path(name, options)?;
            let mut f = File::open(path)?;

            f.read_to_end(&mut buffer)?;
            zip.write_all(&*buffer)?;
            buffer.clear();
        } else if !name.as_os_str().is_empty() {
            // Only if not root! Avoids path spec / warning
            // and mapname conversion failed error on unzip
            println!("adding dir {:?} as {:?} ...", path, name);
            #[allow(deprecated)]
            zip.add_directory_from_path(name, options)?;
        }
    }
    zip.finish()?;
    Result::Ok(())
}

fn get_origin() -> String {
    read_to_string(Path::new(".tease/origin")).expect(&format!("Couldn't read origin"))
}