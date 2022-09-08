use crate::{file_utils::read_branch_head, jwt::JwtToken};
use rocket::serde::{Deserialize, json::Json};
use serde::Serialize;
use tease_common::read::blob_reader::{read_tree_from_commit, collect_objects_from_tree, trail_commits_incl, trail_commits_all};

use super::has_access::{HasAccessRequest, has_access};

#[derive(Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct ObjectCountRequest {
    pub branch: String,
    pub past_origin_head: String,
    pub current_head: String,
}

#[derive(Serialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct ObjectCountResponse {
    pub origin_head: String,
    pub merge_needed: bool,
    pub objects: Vec<String>,
}

#[post("/<user>/<source_name>/what-to-pull", format = "application/json", data="<src_data>")]
pub async fn what_to_pull(
        jwt_token: JwtToken, 
        user: String,
        source_name: String,
        src_data: Json<ObjectCountRequest>
    ) -> Json<ObjectCountResponse>  {
    let mut resp = ObjectCountResponse {
        origin_head: "".to_string(),
        merge_needed: false,
        objects: vec![],
    };

    let has_access_req = HasAccessRequest {
        user: jwt_token.email,
        owner: user.to_string(),
        source_name: source_name.to_string()
    };

    if !has_access(has_access_req, jwt_token.token).await {
        return Json(resp); 
    }
    
    let root_folder = format!("source/{}/{}", user.to_string(), source_name.to_string());
    let branch_head = read_branch_head(&root_folder, &src_data.branch);
    if branch_head.is_err() {
        return Json(resp);
    }

    let head_commit = branch_head.unwrap();
    let temp_solution: Vec<String> = trail_commits_all(root_folder.to_string(), head_commit.to_string())
                                                            .iter()
                                                            .map(|obj| obj.sha1.to_string())
                                                            .collect();
    let objects = get_objects_to_send(root_folder, src_data.past_origin_head.to_string(), head_commit.to_string());

    resp.origin_head = head_commit;
    println!("{}", &src_data.current_head);
    resp.merge_needed = if &src_data.current_head == "# Starting commit" { false } else { !temp_solution.contains(&src_data.current_head) };
    resp.objects = objects;
    
    Json(resp)
}

fn get_objects_to_send(root_folder: String, past_origin_head: String, origin_head: String) -> Vec<String> {
    let mut objects: Vec<String> = vec![]; 

    if past_origin_head == origin_head {
        return objects;
    }

    let mut commits: Vec<String> = trail_commits_incl(root_folder.to_string(), origin_head.to_string(), past_origin_head.to_string())
                                    .iter()
                                    .map(|obj| obj.sha1.to_string())
                                    .collect();
    commits.retain(|commit| commit != "");
    println!("{:?}", commits);

    if commits.is_empty() {
        return objects;
    }

    for commit in commits.iter() {
        objects.push(commit.to_string());
        let tree = read_tree_from_commit(&root_folder, commit);
        objects.push(tree.to_string());

        let mut collected_objects = collect_objects_from_tree(root_folder.to_string(), tree);
        objects.append(&mut collected_objects);
    }

    objects.sort();
    objects.dedup();
    objects
}