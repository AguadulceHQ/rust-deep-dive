// closures can be used as arguments
// we can declare a fn that takes a closure as parameter too
// any function that satisfies the trait bound of the closure can be passed as parameter

// define a fn that takes a generic argument bounded by Fn and calls it
fn call_me<F: Fn()>(f: F) {
    f();
}

// define wrapper function satisfying the Fn bound
fn function() {
    println!("A function that respects the trait bound and will be passed as closure parameter to a fn that calls it");
}

fn main() {
    // define a closure satisfying the Fn bound
    let closure = || println!("This is a closure");

    call_me(closure);
    call_me(function);
}
