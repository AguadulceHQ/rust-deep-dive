// two types of constants that can be declared in any scope (global included)
// const is an unchangeable value
// static, referred to a mut(able) variable it means that accessing or modifying the mutable static variable is unsafe

static LANGUAGE: &str = "Rust";
const THRESHOLD: i32 = 42;

fn is_big(n: i32) -> bool {
    // access gloal constant
    n > THRESHOLD
}

fn main() {
    let n = 16;

    // access constant in main thread
    println!("This is {}", LANGUAGE);
    println!("The threshold is {}", THRESHOLD);
    println!("{} is {}", n, if is_big(n) { "big" } else { "small" });

    // we cannot modify a constant
    // THRESHOLD = 10;
}
