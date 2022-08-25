use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct PullRequest {
    pub objects: Vec<String>,
}