use tease_common::diff::{
    diff_commits, diff_file,
};

pub fn diff_commits_out(a_sha1_commit: String, b_sha1_commit: String) -> () {
    let diff_res = diff_commits(".tease".to_string(), a_sha1_commit.to_string(), b_sha1_commit.to_string());
    if diff_res.is_err() {
        println!("Error {:?}", diff_res.err().unwrap());
        return;
    }
    let diff = diff_res.unwrap();
    for (key, value) in diff.out_map {
        println!("File: {:?}", key);
        for diff_line in value.iter() {
            println!("{}", diff_line);
        }
    }
}


pub fn diff_file_out(a_sha1_blob: String, b_sha1_blob: String) -> () {
    let diff_lines = diff_file(".tease".to_string(), a_sha1_blob.to_string(), b_sha1_blob.to_string());
    for diff_line in diff_lines.iter() {
        println!("{}", diff_line);
    }
}

