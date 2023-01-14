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

#[get("/meet/<name>")]
fn meet(name: &str) -> String {
    format!("You are seeing {}'s agenda", name)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, meet, status])
}
