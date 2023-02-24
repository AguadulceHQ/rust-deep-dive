// closures are functions that can capture the enclosing environment
// |val| val + x this closure captures the x variable
// useful for "on the fly" usage because both input and return types can be inferred

fn main() {
    let outer_env = 42;

    // closures are anonymous, we are binding them to references
    // these nameless fns generally get assigned to appropriately named variables
    let closure_annotated = |i: i32| -> i32 { i + outer_env };
    let closure = |i| i + outer_env;

    // call closures
    println!("closure annoted {}", closure_annotated(1));
    println!("closure inferred {}", closure(1));
    // because of how inference works we cannot now reuse the closure with another type

    // example of closure that takes no arguments and returns an i32
    // the return type is inferred
    let one = || 1;
    println!("Closure returning one {}", one());
}
