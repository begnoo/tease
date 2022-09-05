use serde::Deserialize;


#[derive(Debug, Deserialize)]
pub struct InitResponse {
    #[serde(alias = "ID")] 
    pub id: u64,
    #[serde(alias = "Name")] 
    pub name: String,
    #[serde(alias = "Owner")] 
    pub owner: String,
}