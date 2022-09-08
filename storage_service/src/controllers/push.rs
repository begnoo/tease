use std::{fs::{create_dir_all, remove_file}};
use rocket::{Data, data::ToByteUnit};
use serde::Serialize;
use tease_common::{read::blob_reader::{CommitObject, trail_commits_all, trail_commits_incl}, diff::diff_commits};

use crate::{file_utils::read_branch_head, jwt::JwtToken};

use super::{diff::{count_add_del}, has_access::{HasAccessRequest, has_access}};

#[post("/<user>/<source_name>/push", data = "<src_data>")]
pub async fn push(
        jwt_token: JwtToken,
        user: &str,
        source_name: &str,
        src_data: Data<'_>
    ) -> std::io::Result<String> {

    let has_access_req = HasAccessRequest {
        user: jwt_token.email.to_string(),
        owner: user.to_string(),
        source_name: source_name.to_string()
    };

    if !has_access(has_access_req, jwt_token.token.to_string()).await {
        return Ok("No access.".to_string()); 
    }
    
    let dir_path = format!("source/{}/{}", user, source_name);
    let zip_path = format!("{}/temp_zip", dir_path);

    create_dir_all(&dir_path.to_string()).unwrap();

    src_data.open(128.kibibytes()).into_file(zip_path.to_string()).await?;
    let branch = tease_common::zip_utils::extract_branch_name(zip_path.to_string(), dir_path.to_string());

    let head_commit_res = read_branch_head(&dir_path.to_string(), &branch.to_string());
    let head_commit = if head_commit_res.is_err() { "#".to_string() } else { head_commit_res.unwrap() };

    tease_common::zip_utils::extraxt(zip_path.to_string(), dir_path.to_string());
    remove_file(zip_path.to_string())?;
    
    let props = PushStatsProps {
        email: jwt_token.email,
        token: jwt_token.token,
        user: user.to_string(),
        prev_head_commit: head_commit,
        source_name: source_name.to_string(),
        root_folder: dir_path.to_string(),
        branch,
    };

    push_stats(props).await;

    Ok(format!("Uploaded files for {}/{}", user, source_name))
}

struct PushStatsProps {
    email: String,
    token: String,
    user: String,
    prev_head_commit: String,
    source_name: String,
    root_folder: String,
    branch: String,
}

//stats servis pogleda da li vec postoje, ako posotoje i branch je master -> update
// ako ne postoje tada ide -> create
async fn push_stats(props: PushStatsProps) {
    let head_commit_res = read_branch_head(&props.root_folder.to_string(), &props.branch.to_string());
    let curr_head_commit = head_commit_res.unwrap();

    let objects: Vec<CommitObject>;
    if props.prev_head_commit == "#" {
        objects = trail_commits_all(props.root_folder.to_string(), curr_head_commit.to_string());
    } else {
        objects = trail_commits_incl(props.root_folder.to_string(), curr_head_commit.to_string(), props.prev_head_commit.to_string())
    }

    let items = stats_from_commits(&objects, &props);
    let req_body = PushStatRequests { items };

    let url = format!("http://localhost:8083/commits/multi");
    let client = reqwest::Client::new();
    client.post(url)
        .header("Authorization", format!("Bearer {}", props.token.to_string()))
        .json(&req_body)
        .send()
        .await
        .expect("Couldn't get response")
        .json::<rocket::serde::json::Value>()
        .await
        .expect("Couldn't decode...");
}


#[derive(Debug, Serialize)]
struct PushStatRequests {
    pub items: Vec<PushStatRequest>,
}

fn stats_from_commits(objects: &Vec<CommitObject>, props: &PushStatsProps) -> Vec<PushStatRequest> {
    let mut reqs: Vec<PushStatRequest> = vec![];
    for next in objects.iter() {
        let diff_res = diff_commits(props.root_folder.to_string(), next.sha1.to_string(), next.parents[0].to_string());
        let diff = diff_res.unwrap();
        let items: Vec<(usize, usize)> = diff.out_map
                .iter()
                .map(|(_, value)| count_add_del(value))
                .collect();
        let mut added: usize = 0;
        let mut deleted: usize = 0;
        for (add, del) in items.iter() {
            added = added + add;
            deleted = deleted + del;
        }
        reqs.push(
            PushStatRequest {
                added,
                deleted,
                created_at: next.date,
                user: props.email.to_string(),
                owner: props.user.to_string(),
                source: props.source_name.to_string(),
                sha: next.sha1.to_string(),
                branch: props.branch.to_string()
            }
        );
    }
    
    reqs
}

#[derive(Debug, Serialize)]
struct PushStatRequest {
    #[serde(alias = "createdAt")]
    pub created_at: u64,
    pub added: usize,
    pub deleted: usize,
    pub owner: String,
    pub user: String,
    pub source: String,
    pub sha: String,
    pub branch: String
}

// posaljem na stats async