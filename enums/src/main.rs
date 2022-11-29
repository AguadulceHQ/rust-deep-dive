#[derive(Debug)]
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
    let member_two = Roles::Backend(String::from("Rust, Rocket"));
    let member_three = Roles::Fullstack(String::from("React, Rust"), 42);

    member_one.greeting();
    member_two.greeting();
    member_three.greeting();

    let monster = Some(666);
    // Rust can't infer the type of the Option from the value None so we need to specify it
    let unknown_monster: Option<i32> = None;

    println!("Option and its variants are type of Enum defined in the std library 🤓");
    println!("The beauty of it is that Option and its variants are part of the prelude 😲");
    println!("And in fact we can use it write away both knowns and unknowns 👹");
    dbg!(monster);
    dbg!(unknown_monster);
}
