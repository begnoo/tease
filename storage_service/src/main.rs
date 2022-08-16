#[macro_use] extern crate rocket;
use std::fs::{create_dir_all, remove_file};

use rocket::{Data, data::ToByteUnit};
mod zip_utils;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[post("/<user>/<source_name>", data = "<src_data>")]
async fn push(user: &str, source_name: &str, src_data: Data<'_>) -> std::io::Result<String> {
    let dir_path = format!("source/{}/{}", user, source_name);
    let zip_path = format!("{}/temp_zip", dir_path);

    create_dir_all(&dir_path.to_string()).unwrap();

    src_data.open(128.kibibytes()).into_file(zip_path.to_string()).await?;
    zip_utils::extraxt(zip_path.to_string(), dir_path);
    remove_file(zip_path.to_string())?;
    
    Ok(format!("Uploaded files for {}/{}", user, source_name))
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/source", routes![push])
}
