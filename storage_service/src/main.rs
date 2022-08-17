#[macro_use] extern crate rocket;

mod push;
mod zip_utils;
mod file_utils;
mod jwt;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/source", routes![push::push])
        .mount("/source", routes![push::can_push])
}
