// numeric literals can be type annotated with a suffix e.g. 42i32

// by default the compiler will use i32 for integers and f64 for floating point numbers if no type/constaint exist

fn main() {
    let x = 1u8;
    let y = 2u32;
    let z = 3f32;

    // unsuffixed literals, type depends on how they are used
    let i = 1;
    // f64 requires 8 bytes
    let f = 1.0;

    // `size_of_val` returns the size of a variable in bytes
    println!("size of `x` in bytes: {}", std::mem::size_of_val(&x));
    println!("size of `y` in bytes: {}", std::mem::size_of_val(&y));
    println!("size of `z` in bytes: {}", std::mem::size_of_val(&z));
    println!("size of `i` in bytes: {}", std::mem::size_of_val(&i));
    println!("size of `f` in bytes: {}", std::mem::size_of_val(&f));
}
