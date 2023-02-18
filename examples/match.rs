// pattern matching is done with match kw
// similar to switch in C
// the first matching arm is evaluated and all possible values must be covered

fn main() {
    let number = 13;

    println!("Tell me about your favorite {}", number);

    match number {
        // match single value
        1 => println!("One"),
        // match several values
        2 | 3 | 5 | 7 | 11 => println!("This is a prime"),
        // match an inclusive range
        13..=19 => println!("A teen"),
        // handle rest of cases
        _ => println!("Ain't that special"),
    }

    let boolean = true;
    // match is an expression too so it can be assigned
    let binary = match boolean {
        false => 0,
        true => 1,
    };

    println!("{} -> {}", boolean, binary);
}
