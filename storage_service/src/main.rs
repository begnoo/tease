#[macro_use] extern crate rocket;

mod controllers;

mod file_utils;
mod jwt;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/source", routes![controllers::push::push])
        .mount("/source", routes![controllers::can_push::can_push])
        .mount("/source", routes![controllers::what_to_pull::what_to_pull])
        .mount("/source", routes![controllers::pull::pull])
        .mount("/source", routes![controllers::clone::clone])
        .mount("/source", routes![controllers::clone::clone_branch])
        .mount("/source", routes![controllers::init::init])
        .mount("/read", routes![controllers::read::read_tree])

}
