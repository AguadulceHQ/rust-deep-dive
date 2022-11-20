use std::io;

fn main() {
    println!("Welcome to the 'Guess a Number show' 🧙");

    println!("Your turn. Guess a number 🔮");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Oops, I wasn't able to read your guess 😱");

    println!("You guessed: {guess}");
}
