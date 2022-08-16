use std::fmt::{Display, Formatter, Result};

use super::blob_reader::read_object;


#[derive(Debug)]
pub struct Line {
    pub content: String,
    pub number: usize
}

impl Display for Line {
    fn fmt(&self, f: &mut Formatter) -> Result {
        writeln!(f, "{}", self.content)
    }
}

pub fn get_content_from_blob(blob: String) -> Vec<Line> {
    let parts: Vec<&str> = blob.split("\0").collect();
    let content: Vec<String> = parts[1].to_string()
                            .split("\n")
                            .map(|line| line.trim().to_string())
                            .collect();
    
    content.iter()
            .enumerate()
            .map(|(index, line)| Line {content: line.trim().to_string(), number: index + 1})
            .collect()
}

pub fn get_content_from_sha1(sha1: String) -> Vec<Line> {
    let blob = read_object(&sha1);
    get_content_from_blob(blob)
}
