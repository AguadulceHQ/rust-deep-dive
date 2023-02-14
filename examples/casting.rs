// Rust doesn't provide implicit type conversion (coercion) between primitive types
// explicit conversion can be done with the as keyword (casting)

// Suppress all warnings from casts which overflow.
#![allow(overflowing_literals)]

fn main() {
    let decimal = 42.4321_f32;

    let integer = decimal as u8;
    let character = integer as char;

    // but we can't do let integer: u8 = decimal;
    // let character = decimal as char;

    println!("Casting: {} -> {} -> {}", decimal, integer, character);

    // if we cast a value to an unsigned type
    // T::MAX + 1 is added or subtracted until the value fits into the new type

    // 1000 - 256 - 256 - 256 = 232
    // the first 8 least significant bits (LSB) are kept
    // rest towards the most significant bit (MSB) get trucated

    println!("1000 as a u8 is {}", 1000 as u8);
    // -1 + 256 = 255
    println!("-1 as a u8 is {}", (-1i8) as u8);

    // when casting to signed type, the bitwise result is the same as first casting to the corresponding unsigned type
    // if the most significant bit of that value is 1, then the value is negative

    // and the value of 232 in 8-bit two's complement representation is -24
    println!(" 232 as a i8 is : {}", 232 as i8);

    // saturating cast from float to int
    // if the floating point value exceeds the upper bound or is less than the lower bound, the returned value will be equal to the bound crossed

    // 300.0 as u8 is 255
    println!(" 300.0 as u8 is : {}", 300.0_f32 as u8);
    // -100.0 as u8 is 0
    println!("-100.0 as u8 is : {}", -100.0_f32 as u8);
    // nan as u8 is 0
    println!("   nan as u8 is : {}", f32::NAN as u8);
}
