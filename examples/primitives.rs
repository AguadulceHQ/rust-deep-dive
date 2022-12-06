// Rust provides access to a variety of types

/* Scalar types

* signed integers: i8, i16, i32, i64, i128 and isize (pointer size)
* unsigned integers: u8, u16, u32, u64, u128 and usize (pointer size)
* floating point: f32, f64
* char Unicode scalar for 'a', 'α' and '∞' (4 bytes each)
* bool either true or false
* unit type (), only possible value is empty tuple (it's not compound because it cannot contain multiple values)
*/

/* Compound types

* arrays homogeneous list of elements like [1, 2, 3]
* tuples heterogeneous list like (1, true)
*/

// integers default to i32, floats to f64
// Rust can infer type from context

fn main() {
    // variables can be type annoted
    let projectDone: bool = true;

    let a_float: f64 = 1.0; // regular annotation
    let an_integer = 5i32; // suffix annotation (less common)

    // default
    let default_float = 3.0; //f64

    // infer from context
    let mut inferred = 12; // type is i64 inferred from next line;
    inferred = 4294967296i64; // suffix annotation

    // a mutable can change value
    let mut changes = 12;
    changes = 42;

    // override a variable by using overshadowing
    let changes = true;
}
