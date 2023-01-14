#[macro_use]
extern crate rocket;

// mounts index route to / path
#[get("/")]
fn index() -> &'static str {
    "Hello, folks! 👋"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
