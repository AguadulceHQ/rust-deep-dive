// From and Into traits are linked
// if you are able to convert type A from type B then it should be easy to go from B to A

// From allows a type to define how to create itself from another type
// Into is reciprocal to From if you implement the From trait for your type, Into calls it when neessary

use std::convert::From;

// TryFrom and TryInto are instead used for fallible conversion
// because of the potential failure they return a Result

use std::convert::TryFrom;
use std::convert::TryInto;

#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}

#[derive(Debug, PartialEq)]
struct EvenNumber(i32);

impl TryFrom<i32> for EvenNumber {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value % 2 == 0 {
            Ok(EvenNumber(value))
        } else {
            Err(())
        }
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

    // TryFrom
    assert_eq!(EvenNumber::try_from(8), Ok(EvenNumber(8)));
    assert_eq!(EvenNumber::try_from(5), Err(()));

    // Try Into
    let result: Result<EvenNumber, ()> = 42i32.try_into();
    assert_eq!(result, Ok(EvenNumber(42)));
    let result: Result<EvenNumber, ()> = 5i32.try_into();
    assert_eq!(result, Err(()));
}
