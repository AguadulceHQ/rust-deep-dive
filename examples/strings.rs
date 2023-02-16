// to convert any type to a String we implement the ToString trait for the type
// instead of doing this directly we implement fmt::Dsiplay which automagically provides ToString and allows to use print! macro

use std::fmt;

struct Circle {
    radius: i32,
}

impl fmt::Display for Circle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Circle of radius {}", self.radius)
    }
}

fn main() {
    let circle = Circle { radius: 42 };
    println!("{}", circle.to_string());

    // to instead convert a string into a Number we generally use the parse fn
    // either to arrange for type inference or to specify the type to parse using the turbofish syntax

    // to obtain this behaviour on user defined types you need to implement the FromStr trait
    let parsed: i32 = "5".parse().unwrap();
    let turbo_parsed = "42".parse::<i32>().unwrap();

    let sum = parsed + turbo_parsed;
    println!("Sum is {:?}", sum);
}
