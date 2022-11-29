enum Roles {
    Frontend(String),
    Backend(String),
    Fullstack(String, u8),
}

impl Roles {
    fn greeting(&self) {
        println!("Howdy here is a method from the enum 👋");
    }
}

fn main() {
    println!("Welcome to defining enums 🔢");

    let member_one = Roles::Frontend(String::from("React"));
    member_one.greeting();
}
