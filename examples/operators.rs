// integers, floats, characters, strings, booleans and unit type can be expressed as literals

// integers can be expressed with hex, octal or binary notation
// 0x, 0o, 0b

// we can use underscores to improve readability 1_000 or 0.000_001 = 0.000001

// we need to inform the compiler the type of literals we want to use

fn main() {
    // addition of integers
    println!("2 + 2 = {}", 2u32 + 2);

    // substraction of integers
    println!("2 - 3 = {}", 2i32 - 2);

    // short-circuited boolean logic
    println!("true AND false is {}", true && false);
    println!("true OR false is {}", true || false);
    println!("NOT true is {}", !true);

    // bitwise operations
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);

    // underscores improve readibility
    println!("One billion is written as {}", 1_000_000_000u32);
}
