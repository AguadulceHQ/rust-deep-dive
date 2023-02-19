fn main() {
    let triple = (-1, 0, 1);

    println!("My triplet is {:?}", triple);

    match triple {
        // match can be used to destructure a tuple type
        (-1, y, z) => println!(
            "We know the first is -1 (harcoded) the rest instead are 2nd is {:?} and 3rd is {:?}",
            y, z
        ),
        (-1, ..) => println!("First is 1 and the rest doesn't matter"),
        (.., 2) => println!("Last is 2 and the rest doesn't matter"),
        (3, .., 4) => println!("First is 3 and last is 4, the rest doesn't matter"),
        // _ means "don't" bind values to a variable
        _ => println!("You aren't any of the above, it doesn't matter who you are"),
    }
}
