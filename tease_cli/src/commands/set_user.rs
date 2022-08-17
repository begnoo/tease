use std::path::Path;

use tease_common::write::bolb_writer::create_tease_file;

pub fn set_user(email: String) {
    create_tease_file(Path::new(".tease/user"), format!("{}", email));
}