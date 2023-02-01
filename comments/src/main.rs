fn main() {
    println!("Welcome to the art of commenting 💭");

    // 42 represents the meaning of life
    let x = 42;

    println!("Can you explain me what this number is about? {} 🤯", x);

    add_twice(x);
}

/*
    This is a comment that spans across multiple line
    add_twice adds a number to itself
    We will discover later a beter way to document functions
*/

fn add_twice(n: i32) -> i32 {
    n + n
}
