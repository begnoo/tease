use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct InitRequest {
    pub name: String,
    pub owner: String,
    pub visability: bool,
}