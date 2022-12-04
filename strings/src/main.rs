fn main() {
    // string literal
    let name = "Luca";
    // promoted to String
    let name = name.to_string();
    // you can also use it directly
    let name = "Luca".to_string();
    // String from a string literal
    let name = String::from("Luca");

    println!(
        "Welcome here in the Strings program. My name is {} and I will be your host 🤝",
        name
    );

    let hello = String::from("السلام عليكم");

    println!(
        "By default Strings in Rust are UTF-8 encoded. We can say hello in Arabic {} 🤯",
        hello
    );

    let mut full_name = String::from("Luca");
    full_name.push_str("Daniel");

    println!("I forgot to tell you my full name {} 🤝", full_name);

    let name = String::from("Luca");
    let last_name = String::from("Daniel");
    let full_name = name + &last_name;

    println!(
        "We can also leverage the + operator to achieve the same result 👉 {}",
        full_name
    );

    let name = String::from("Luca");
    let last_name = String::from("Daniel");
    let full_name = format!("{} {}", name, last_name);

    println!(
        "A cleaner version is done through the format! macro 👉 {}",
        full_name
    );

    println!("Let's do something fun and list all the characters in my name 🤓");

    for c in name.chars() {
        println!("{}", c);
    }

    println!("But Rust under the hood stores a vector of bytes let's look at that 🧠");

    for b in name.bytes() {
        println!("{}", b);
    }
}
