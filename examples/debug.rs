/*
    All types which want to use std::fmt formatting traits require an implementation.
    Automatic implementation are provided for types in std library. For other we need to do the work.

    With fmt::Debug trait any type can derive (and automatically create) a fmt::Debug implementation.

    fmt::Display instead requires a manual implementation
*/

struct UnPrintable(i32);

// derive attribute automatically creates the implementation
// required to make the struct printable with fmt::Debug
#[derive(Debug)]
struct Printable(i32);

// a struct that contains a Printable
#[derive(Debug)]
struct Deep(Printable);

fn main() {
    println!("Printable: {:?} will print", Printable(42));

    // the downside of this is that we don't have control on how this renders
    println!("Deep: {:?} is printable too!", Deep(Printable(42)));
}
