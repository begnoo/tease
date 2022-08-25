use serde::Deserialize;


#[derive(Deserialize, Debug)]
pub struct CanPushResponse {
    pub result: bool,
    pub diff: Vec<String>,
    pub head_commit: String,
    pub present: bool,
    // pub empty: bool,
}