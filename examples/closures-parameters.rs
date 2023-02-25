// Rust chooses how to capture variables on the fly without type annotation (most cases)
// but not for functions
// taking a closure as input parameter the closure's complete type must be annotated using one a few traits
// this is determined by what the closure does with the captured value

// Fn: closure uses captured value by reference &T
// FnMut: closure uses the captured value by mutable reference &mut T
// Fnonce:: closure uses the captured value by value T

// on a var by var basis the compiler will capture variables in the least restrictive manner possible

// e.g. if a parameter is annotated as FnOnce it may capture by &T, &mut T or T but the compiler will choose based on how the captured variables are used in the closure at compile time
// e.g. if a move is possible then any type of borrow should also be possible

// fn that takes a closure as argument and calls it
// <F> denotes that F is a generic type parameter
fn apply<F>(f: F)
where
    F: FnOnce(),
{
    // closure takes no input and returns nothing
    f();
}

// a fn that takes a closure and returns an i32
fn apply_to_3<F>(f: F) -> i32
where
    F: Fn(i32) -> i32,
{
    f(3) // returned value
}

fn main() {
    use std::mem;

    let greeting = "hello";
    // non copy type, to owned creates owned data from borrowed one
    let mut farewell = "goodbye".to_owned();

    // closure that captures two values greeting by reference and farewell by value
    let diary = || {
        println!("I said {}", greeting);

        farewell.push_str("!!!");
        println!("Then I said {}", farewell);
        println!("So that now I can sleep");

        // manually calling drop forces farewell to be captured by value
        // now it requires FnOnce
        mem::drop(farewell);
    };

    // call the function which applies the closure
    apply(diary);

    // double satisfied apply_to_3 trait bound
    let double = |x| 2 * x;

    println!("3 doubled is {}", apply_to_3(double));
}
