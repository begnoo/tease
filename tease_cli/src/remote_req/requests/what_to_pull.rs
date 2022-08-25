use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct ObjectCountRequest {
    pub branch: String,
    pub past_origin_head: String,
    pub current_head: String,
}