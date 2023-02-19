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

    let list = [-1, 0, 1];

    match list {
        [-1, y, z] => println!(
            "We know the first item is -1 then list[1] = {} and list[2] = {}",
            y, z
        ),
        // we can ignore single values
        [-1, _, z] => println!(
            "We know the first item is -1 then list[1] we don't care and list[2] = {}",
            z
        ),
        // we can bind some and ignore the rest
        [-1, y, ..] => println!(
            "We know the first item is -1 then list[1] = {} and we don't care about the rest",
            y
        ),
        // we can also store in another array/slice the type will be infered
        [-1, y, tail @ ..] => println!(
            "We know the first item is -1 then list[1] = {} and the rest of whatever is left are {:?}",
            y, tail
        ),
        // we can be more creative store the first, last in variables and the rest into an array
        [min, middle @ .., max] => println!("We extrapolate min = {} and max {} from the first and last position then the rest is in an array {:?}", min, max, middle),
    }
}
