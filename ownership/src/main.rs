fn main() {
    let x = 42;
    let y = x;

    println!(
        "The size of x and y is known at compile time, they are both stored on the stack x: {} y: {} 🔝", x, y
    );

    let string_literal = String::from("I am a string literal I can't change");

    println!(
        "Now let's use a string literal a variable that we can't change 👉 {} ❗",
        string_literal
    );

    let string_copy = string_literal;

    println!("string_copy is just a pointer that points to the heap contents of string_literal no deep copy here 👉 {} 🧠", string_copy);

    let string_gets_stolen = String::from("A string coming from main says: Bye bye folks going into the function and not getting back 😞");

    take_ownership(string_gets_stolen);

    // we won't be able to access string_gets_stolen here!

    let string_gets_stolen_but_is_returned = String::from(
        "A string coming from main and going to fn but I'll see you guys once back! 🤗",
    );

    // bonkers we are also using shadowing here
    let string_gets_stolen_but_is_returned =
        take_ownership_but_gives_it_back(string_gets_stolen_but_is_returned);

    println!("Wow wow we are able to access our string despite being passed to a fn because it was returned to us before the fn ended. Here it is 👉 {} 🎩", string_gets_stolen_but_is_returned);

    let (my_name, my_name_length) = calculate_length(String::from("Luca"));

    println!("Did you know that my name is {} and that's {} characters long? And that we just returned multiple values using a tuple 😮", my_name, my_name_length);

    fn take_ownership(your_string: String) {
        println!("take_ownership fn takes a string and doesn't give it back so main can't access it anymore because Rust will free the memory once out of scope. Here it is 👉 {} 💸", your_string);
    }

    fn take_ownership_but_gives_it_back(your_string: String) -> String {
        println!("take_ownership_but_gives_it_back fn takes a string and gives it back so main can still access it after the function is done. Here it is 👉 {} 🤯", your_string);
        // this is an expression
        your_string
    }

    fn calculate_length(name: String) -> (String, usize) {
        let length = name.len();

        (name, length)
    }
}
