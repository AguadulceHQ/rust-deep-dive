#[macro_use]
extern crate rocket;

// mounts index route to / path
#[get("/")]
fn index() -> &'static str {
    "Hello, folks! 👋"
}

#[get("/status")] // <- route attribute
fn status() -> &'static str {
    // <- request handler
    "We are live 🔥"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, status])
}
