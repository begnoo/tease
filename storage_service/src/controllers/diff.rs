use tease_common::diff::{diff_commits, DiffLine, PlainDisplay};
use rocket::serde::{Serialize, Deserialize, json::Json};


#[derive(Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct DiffCommits {
    pub commit: String,
    #[serde(rename(deserialize = "parentCommit"))]
    pub parent_commit: String,
}

#[derive(Serialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct DiffFile {
    pub path: String,
    pub diff: String,
    pub added: usize,
    pub deleted: usize
}

#[derive(Serialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct DiffContent {
    pub items: Vec<DiffFile>
}

#[post("/<user>/<source_name>/diff",  data = "<commits>")]
pub async fn read_diff_commits(user: &str, source_name: &str, commits: Json<DiffCommits>) -> Option<Json<DiffContent>> {
    let root_folder = format!("source/{}/{}", user, source_name);

    let diff_res = diff_commits(root_folder.to_string(), commits.commit.to_string(), commits.parent_commit.to_string());
    if diff_res.is_err() {
        println!("{:?}", diff_res.err().unwrap());
        return None;
    }

    let diff = diff_res.unwrap();
    let items: Vec<DiffFile> = diff.out_map.iter()
                .map(|(key, value)| {
                    let counts = count_add_del(value);
                    DiffFile { 
                        path: key.to_string(), 
                        diff: format_diff_lines(value),
                        added: counts.0,
                        deleted: counts.1
                    } 
                }).collect();

    Some(Json(DiffContent {items}))
}

fn count_add_del(lines: &Vec<DiffLine>) -> (usize, usize) {
    let add_count = lines.iter().filter(|line| line.state == "add").count();
    let del_count = lines.iter().filter(|line| line.state == "del").count();
    
    (add_count, del_count)
}

fn format_diff_lines(lines: &Vec<DiffLine>) -> String {
    let formated_lines: Vec<String> = lines.iter().map(|line| line.plain_string()).collect();
    
    formated_lines.join("\n")
}