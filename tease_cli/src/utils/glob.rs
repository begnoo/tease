use glob::glob;

pub fn get_all_repo_paths() -> Vec<String> {
    
    let path_entries = glob("./**/*").expect("Failed to read glob pattern");
    path_entries.into_iter().map(|entry| entry.unwrap()
                                            .to_str()
                                            .unwrap()
                                            .to_string()
                                            .replace("\\", "/"))
                            .filter(|entry| !entry.contains(".tease") && !entry.contains("tease_cli.exe"))
                            .collect()
}