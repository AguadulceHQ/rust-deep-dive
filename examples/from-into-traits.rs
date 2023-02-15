// From and Into traits are linked
// if you are able to convert type A from type B then it should be easy to go from B to A

// From allows a type to define how to create itself from another type
// Into is reciprocal to From if you implement the From trait for your type, Into calls it when neessary

use std::convert::From;

#[derive(Debug)]

struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}

fn main() {
    // convert a u32 into a Number through From trait
    let num = Number::from(30);
    println!("My number is {:?}", num);

    let int = 42;
    // because From is implemented on Number this is allowed
    let num: Number = int.into();
    println!("My number is {:?}", num);
}
