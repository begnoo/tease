use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct ObjectCountResponse {
    pub origin_head: String,
    pub merge_needed: bool,
    pub objects: Vec<String>,
}