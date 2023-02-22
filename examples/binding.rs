// match provides @ sigil for binding values to names

// a fn that returns a u32
fn age() -> u32 {
    18
}

fn some_number() -> Option<u32> {
    Some(42)
}

fn main() {
    println!("In which bucket are you?");

    match age() {
        0 => println!("I haven't celebrated my first birthday yet"),
        // we would like to get the matched value to print it
        n @ 1..=12 => println!("I'm a child of age {:?}", n),
        n @ 13..=19 => println!("I'm a teen of age {:?}", n),
        // nothing bound return result
        n => println!("I am an old person of age {:?}", n),
    }

    match some_number() {
        // Some variant match if its value, bound to n is equal to 42
        Some(n @ 42) => println!("The answer is {}", n),
        // Match other number
        Some(n) => println!("Not interesting {}", n),
        // match anything else e.g. None variant
        _ => (),
    }
}
