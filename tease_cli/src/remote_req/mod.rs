pub mod login;
pub mod post_push;
pub mod can_push;
pub mod what_to_pull;
pub mod post_pull;
pub mod get_clone;
pub mod clone_branch;
pub mod post_init;
pub mod requests;
pub mod responses;

// const STORAGE_SERVICE: &str = "http://localhost:8000/source";
const AUTH_SERVICE: &str = "http://localhost:8080/auth";
const REPO_SERVICE: &str = "http://localhost:8081/source";
