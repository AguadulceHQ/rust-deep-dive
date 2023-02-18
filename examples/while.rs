// run a loop while a condition is true

fn main() {
    // counter
    let mut n = 1;

    // loop while n is less than 101, this plays the game 100 times

    while n < 101 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }

        // increment counter
        n += 1;
    }
}
