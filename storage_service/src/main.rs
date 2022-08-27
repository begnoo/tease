use rocket::{fairing::{Fairing, Info, Kind}, http::Header, Request, Response};

#[macro_use] extern crate rocket;

mod controllers;

mod file_utils;
mod jwt;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Cors)
        .mount("/source", routes![controllers::push::push])
        .mount("/source", routes![controllers::can_push::can_push])
        .mount("/source", routes![controllers::what_to_pull::what_to_pull])
        .mount("/source", routes![controllers::pull::pull])
        .mount("/source", routes![controllers::clone::clone])
        .mount("/source", routes![controllers::clone::clone_branch])
        .mount("/source", routes![controllers::init::init])
        .mount("/source", routes![all_options])
        .mount("/read", routes![controllers::read::read_tree])
        .mount("/read", routes![controllers::read::read_blob])
        .mount("/read", routes![controllers::read::read_branch])
        .mount("/read", routes![controllers::read::read_branches])
        .mount("/read", routes![all_options])

}

#[options("/<_..>")]
fn all_options() {
    /* Intentionally left empty */
}

pub struct Cors;

#[rocket::async_trait]
impl Fairing for Cors {
    fn info(&self) -> Info {
        Info {
            name: "Cross-Origin-Resource-Sharing Fairing",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "POST, PATCH, PUT, DELETE, HEAD, OPTIONS, GET",
        ));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}
