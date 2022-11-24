fn main() {
    println!("Welcome to flow control 🚦");

    println!("Let's first try an if/else type of situation");

    let am_i_smart = true;

    if am_i_smart {
        println!("If I am smart this will get printed. That definitely worked 🤓");
    } else {
        println!("Something went horribly wrong 😞");
    }

    let number = 42;

    if number == 42 {
        println!(
            "We can also have single if statements like this that executes if the number is {}",
            number
        );
    }

    let best_number = if number == 42 { 42 } else { 0 };

    println!("We can also assign variables leveraging the if expression as it can return a value like {} 🔝", best_number);

    let mut countdown = 3;

    println!("We can also repeating the same chunk of code several times with loop, while and for constructs 🔁");

    loop {
        println!("And...{} 💫", countdown);
        countdown -= 1;
        if countdown < 1 {
            break;
        }
    }

    println!("Done with repetition with the loop keyword, let's try the same with a while 😏");

    while countdown < 3 {
        println!(
            "I am resetting the counter to the original value, now it is {}",
            countdown,
        );
        countdown += 1;
    }

    println!("Gosh finally we are back! Countdown is {} ✅", countdown);

    println!("Let's try again with a for loop 🤓");

    for number in (1..countdown + 1).rev() {
        println!("And...{} 💫", number);
    }
}
