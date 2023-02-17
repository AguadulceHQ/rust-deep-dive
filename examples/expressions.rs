// A Rust program is mostly made up of statements
// most common are declaring a variable binding and using ; with an expression

fn main() {
    // variable binding expression
    let x = 42;

    // expression;
    x;
    x + 1;
    42;

    // blocks are also expressions
    // the last expression in a block will be assigned as a value
    // if the last expression block ends with a semicolon the return value will be ()

    let y = 42u32;

    let z = {
        let y_squared = y * y;
        let y_cube = y_squared * y;

        // this expression will be assigned to z
        y_cube + y_squared + y
    };

    let w = {
        // the semicolon suprresses the expression and () is assigned to z
        2 * y;
    };

    println!("x is {:?}", x);
    println!("y is {:?}", y);
    println!("z is {:?}", z);
}
