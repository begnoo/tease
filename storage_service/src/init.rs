use std::{fs::create_dir_all, path::Path};

use rocket::serde::json::Json;
use serde::Serialize;

#[derive(Serialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct BoolResult {
    pub result: bool,
}

#[get("/init/<user>/<source_name>", format = "application/json")]
pub async fn init(
        user: &str,
        source_name: &str,
    ) -> Json<BoolResult> {

    let mut resp = BoolResult {
        result: false,
    };
    
    let root_folder = format!("source/{}/{}", user, source_name);
    let res = create_dir_all(Path::new(&root_folder));
    
    if res.is_err() {
        return Json(resp);
    }

    resp.result = true;

    Json(resp)
}