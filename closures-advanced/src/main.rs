// takes a i32 and increments it by one
fn increment(num: i32) -> i32 {
    num + 1
}

// takes function pointer to any function that takes i32 and returns i32
// takes also one i32 value
// calls f twice, passes to it x and adds the result coming from the two calls
fn run_it_twice(f: fn(i32) -> i32, x: i32) -> i32 {
    f(x) + f(x)
}

fn main() {
    let computation = run_it_twice(increment, 1);

    println!("The results should be 4 and it is {}", computation);
}
