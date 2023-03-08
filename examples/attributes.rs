// metadata applied to some module, crate or item
// generally used for conditional compilation of code, set crate name, disable lints, enable compiler features etc.
// mark fn as unit tests, or part of a benchmark

// #![crate_wide_attribute]

// #[attribute(value)]

#[derive(Debug)]
struct Project {
    id: u32,
}

// configure conditional checks

// gets compiled only if target is linux
#[cfg(target_os = "linux")]
fn on_linux() {
    println!("Running on Linux");
}

#[cfg(not(target_os = "linux"))]
fn on_linux() {
    println!("Not running on Linux");
}

fn main() {
    let p = Project { id: 3 };
    println!("{:?}", p);
    on_linux();

    // evaluates to true/false
    if cfg!(target_os = "linux") {
        println!("yes this is Linux");
    } else {
        println!("Not running on Linux")
    }

    conditional_function();
}

// attributes can take multiple values
// #[attribute(value, value2)]

// attribute that disables the dead_code lint
#[allow(dead_code)]
fn unused_function() {}

// custom attribute
// can be called rustc --cfg some_condition attributes.rs
#[cfg(some_condition)]
fn conditional_function() {
    println!("condition met");
}
