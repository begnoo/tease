#[macro_use] extern crate rocket;

mod has_access;

mod can_push;
mod push;

mod what_to_pull;
mod pull;

mod clone;

mod file_utils;
mod jwt;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/source", routes![push::push])
        .mount("/source", routes![can_push::can_push])
        .mount("/source", routes![what_to_pull::what_to_pull])
        .mount("/source", routes![pull::pull])
        .mount("/source", routes![clone::clone])

}
