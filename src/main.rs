use std::env;

use rocket::fs::{relative, FileServer};
use rocket_dyn_templates::{context, Template};

#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> Template {
    Template::render("home/index", context! {})
}

#[launch]
fn rocket() -> _ {
    let path = get_or_else(
        String::from("STATICS_PATH"),
        String::from(relative!("/static")),
    );
    let statics = FileServer::from(path);

    rocket::build()
        .mount("/", routes![index])
        .mount("/static", statics)
        .attach(Template::fairing())
}

fn get_or_else(key: String, default: String) -> String {
    match env::var(key) {
        Ok(value) => value,
        Err(_) => default,
    }
}
