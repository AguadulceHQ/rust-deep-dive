// variable bindings are immutable by default but this can be overriden with the mut modifier

fn main() {
    let _immutable = 1;
    let mut mutable = 1;

    println!("mutable is {}", mutable);
    mutable += 1;

    println!("mutable is now {} after the mutation", mutable);

    // we can't do _immutable += 1;
}
