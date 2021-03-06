use std::fs;
use std::path::Path;


use crate::utils::blob_writer::{create_tease_file, create_tease_folder, create_index_file};

pub fn create_repo(repo_name: String) -> String {

    let path_buff = Path::new(&repo_name).join(".tease");
    if path_buff.as_path().exists() {
        return format!("Folder with name {:?} already exists.", repo_name).to_string();
    }

    match fs::create_dir_all(path_buff.as_path()) {
        Ok(_folder) => {

            create_index_file(Path::new(&repo_name).join(".tease").join("index").as_path());
            create_tease_file(Path::new(&repo_name).join(".tease").join("HEAD").as_path(), "refs/heads/master".to_string());
            create_tease_file(Path::new(&repo_name).join(".tease").join("log").as_path(), "# Commit log".to_string());
            
            create_tease_folder(Path::new(&repo_name).join(".tease").join("objects").as_path());

            create_tease_folder(Path::new(&repo_name).join(".tease").join("refs").as_path());
            create_tease_folder(Path::new(&repo_name).join(".tease").join("refs").join("heads").as_path());
            create_tease_file(Path::new(&repo_name).join(".tease").join("refs").join("heads").join("master").as_path(), "# Starting commit".to_string());

            format!("tease cli successfully created a new repo named {:?}", repo_name).to_string()
        },
        Err(error) => format!("Problem creating the folder: {0}", error.to_string()).to_string(),
    }

}