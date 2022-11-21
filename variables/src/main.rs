fn main() {
    let x = 0;
    println!("The value of x is {x}");
    println!("'let x: i32 = 0;' is immutable by default so you can't change it ❌");

    let mut y = 0;
    println!("The value of y is {y}");
    println!("'let mut y: i32 = 0;' is a mutable so you can change it ✅");
    y = 1;
    println!("The new value of y is {y}");
}
