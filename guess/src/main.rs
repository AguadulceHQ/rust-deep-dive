use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Welcome to the 'Guess a Number show' 🧙");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Your turn. Guess a number 🔮");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Oops, I wasn't able to read your guess 😱");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed 👉 {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("You need to be more ambitious 👎"),
            Ordering::Greater => println!("A little more humble next time 😵‍💫"),
            Ordering::Equal => {
                println!("Congratulations! Where should we wire the mulah? 💰");
                break;
            }
        }
    }
}
