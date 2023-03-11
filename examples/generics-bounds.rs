// type parameters must use traits as bounds to stipulate what functionality a type should implement

// define a fn that takes a generic type T that must implement the Display trait
use std::fmt::Display;

fn printer<T: Display>(t: T) {
    println!("{}", t);
}

fn main() {
    printer("Hello, World");
}
