use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct CanPushRequest {
    pub branch: String,
    pub sha1: String,
    pub objects: Vec<String>,
}