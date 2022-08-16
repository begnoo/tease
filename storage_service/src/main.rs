#[macro_use] extern crate rocket;

mod push;
mod zip_utils;
mod jwt;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}



#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/source", routes![push::push])
        .mount("/source", routes![push::can_push])
}
