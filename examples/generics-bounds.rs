// type parameters must use traits as bounds to stipulate what functionality a type should implement

// define a fn that takes a generic type T that must implement the Display trait
use std::fmt::Display;

fn printer<T: Display>(t: T) {
    println!("{}", t);
}

// you can also specify on a struct level
struct S<T: Display>(T);

// a trait that implements the print market {:?}
use std::fmt::Debug;

trait HasArea {
    fn area(&self) -> f64;
}

impl HasArea for Rectangle {
    fn area(&self) -> f64 {
        self.length * self.height
    }
}

#[derive(Debug)]
struct Rectangle {
    length: f64,
    height: f64,
}
#[allow(dead_code)]
struct Triangle {
    length: f64,
    height: f64,
}

// the generic T must implement Debug, regardless of the specific type the fn should work if that's true
fn print_debug<T: Debug>(t: &T) {
    println!("{:?}", t);
}

// T must implement HasArea
fn area<T: HasArea>(t: &T) -> f64 {
    t.area()
}

fn main() {
    printer("Hello, World");

    let rectangle = Rectangle {
        length: 3.0,
        height: 4.0,
    };
    print_debug(&rectangle);
    println!("Area: {}", rectangle.area());
}
