// foreign function interface (FFI)
extern "C" {
    fn abs(input: i32) -> i32;
}

fn main() {
    // fn declared in extern blocks are always unsafe to call from Rust
    // other language do not have same rules and guarantees so we need to ensure safety
    unsafe {
        println!("Absolute value of -42 according to C: {}", abs(-42));
    }
}

// disable defult mangling so that the other language can access the human readable signature
#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("Calling a Rust function from C!");
}
