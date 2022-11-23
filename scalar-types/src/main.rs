fn main() {
    let default_integer = 2;
    let min_integer: u32 = 0;
    let max_integer: u32 = 255;

    println!("This variable didn't have a type and defaults to a u32, the value stored is {default_integer} 🦺");
    println!("The min integer we can store in a u32 is {min_integer} ⬇️");
    println!("The max integer we can store in a u32 is {max_integer} ⬆️");

    let default_float = 1.0;
    let min_float: f32 = 0.0;
    let max_float: f32 = 2_147_483_648.0;

    println!(
        "This variable didn't have a type and defaults to a f64, the value stored is {:.1} 🦺",
        default_float
    );
    println!(
        "The min integer we can store in a f32 is {:.1} ⬇️",
        min_float
    );
    println!(
        "The max integer we can store in a f32 is {:.1} ⬆️",
        max_float
    );

    let default_bool = true;
    let truthy_value: bool = true;

    println!(
        "This variable didn't have an explicit type and defaults to a bool because of the boolean assigned to it, the value stored is {} 🦺",
        default_bool
    );

    println!(
        "This variable has an explicit type annotation for a truthy value 👉 {} ✅",
        truthy_value
    );

    println!(
        "We negate the value of the boolean variable to get the opposite with a NOT operator 👉 {} 🤯",
        !truthy_value
    );

    let default_char = 'a';
    let character_value = 'A';
    let emoji = '🔝';

    println!(
      "This variable didn't have an explicit type and defaults to a char because of the character assigned to it, the value stored is {} 🦺",
      default_char 
    );

    println!(
        "This variable has an explicit type annotation for a single character value 👉 {} ✅",
        character_value
    );

    println!(
      "Rust represents the char type with 4 bytes as Unicode Scalar Value so we can print emojis with a char {} this is 🤯",
      emoji
  );
}
