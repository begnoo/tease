use std::io::Write;
use std::fs;
use std::fs::File;
use std::path::Path;


pub fn create_repo(repo_name: String) -> String {

    let path_buff = Path::new(&repo_name).join(".tease");
    if path_buff.as_path().exists() {
        return format!("Folder with name {:?} already exists.", repo_name).to_string();
    }

    match fs::create_dir_all(path_buff.as_path()) {
        Ok(_folder) => {
            create_tease_file(Path::new(&repo_name).join(".tease").join("log").as_path(), "# Commit log")
                .expect(&format!("Couldn't create log file for repo {:?}", repo_name).to_string());

            create_tease_file(Path::new(&repo_name).join(".tease").join("temp").as_path(), "# Temp commit")
                .expect(&format!("Couldn't create log file for repo {:?}", repo_name).to_string());

            create_tease_folder(Path::new(&repo_name).join(".tease").join("objects").as_path())
                .expect(&format!("Couldn't create objects folder for repo {:?}", repo_name).to_string());

            format!("tease cli successfully created a new repo named {:?}", repo_name).to_string()
        },
        Err(error) => format!("Problem creating the folder: {0}", error.to_string()).to_string(),
    }

}


fn create_tease_file(path: &Path, message: &str) -> std::io::Result<()> {
    let mut file = File::create(path)?;
    file.write_all(message.as_bytes())?;

    Ok(())
}

fn create_tease_folder(path: &Path) -> std::io::Result<()> {
    fs::create_dir(path)?;

    Ok(())
}