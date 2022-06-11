use rocket_dyn_templates::{context, Template};

#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> Template {
    Template::render("home/index", context! {})
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .attach(Template::fairing())
}
