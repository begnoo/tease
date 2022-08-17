use std::{fs::read_to_string, path::Path, io};

pub fn read_branch_head(root_folder: &String, branch: &String) -> io::Result<String> {
    read_to_string(&Path::new(root_folder)
        .join("refs")
        .join("heads")
        .join(branch))
}