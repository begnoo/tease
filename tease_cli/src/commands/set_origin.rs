use std::path::Path;
use tease_common::write::bolb_writer::create_tease_file;

pub fn set_origin(origin: String) {
    create_tease_file(Path::new(".tease/origin"), origin.to_string());
}