fn main() {
    let mut num = 42;

    // same memory location where num is stored
    // having a mutable and immutable ref is not allowed in safe Rust's ownership rules
    let raw_pointer_one_immutable = &num as *const i32;
    let raw_pointer_two_mutable = &mut num as *mut i32;

    // creating a raw pointer is not a problem is when we try to access (deref) the value that it points that we may end up with an invalid value
    // this is why we use an unsafe block here
    unsafe {
        println!("Raw pointer one: {}", *raw_pointer_one_immutable);
        println!("Raw pointer two: {}", *raw_pointer_two_mutable);
    }
}
