#[derive(Debug)]
pub struct Line {
    pub content: String,
    pub number: usize
}

pub fn get_content_from_blob(blob: String) -> Vec<Line> {
    let parts: Vec<&str> = blob.split("\0").collect();
    let content: Vec<String> = parts[1].to_string()
                            .split("\n")
                            .map(|line| line.to_string())
                            .collect();
    
    content.iter()
            .enumerate()
            .map(|(index, line)| Line {content: line.trim().to_string(), number: index + 1})
            .collect()
}