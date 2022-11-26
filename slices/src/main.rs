fn main() {
    let full_name = String::from("Luca Daniel");
    let full_name_literal = "Luca Daniel";

    let first_name = &full_name[..4];
    let last_name = &full_name[5..];

    println!(
        "Slices are a breeze my full name is {}, my name is {}, my last name is {} 🪚",
        full_name, first_name, last_name
    );

    // equivalent to slice of the entire string
    let sliced_first_name = slice_first_name(&full_name);
    let sliced_first_name_literal = slice_first_name(full_name_literal);

    println!("We are slicing thanks to our flexible fn API the first name from a String and also from a string literal 🔝 {} {}", sliced_first_name, sliced_first_name_literal);

    let numbers = [0, 17, 42, 100];

    let favorite_numbers = &numbers[1..3];

    assert_eq!(favorite_numbers, &[17, 42]);

    println!("If I can 🖨️  this it means that my favorite numbers are 17 and 42 👏");

    // we accept &str so that it works both for String literals and slices!
    fn slice_first_name(full_name: &str) -> &str {
        let bytes = full_name.as_bytes();

        for (i, &character) in bytes.iter().enumerate() {
            if character == b' ' {
                return &full_name[0..i];
            }
        }

        &full_name[..] // returns a slice of the entire name that gets assigned to name_slice
    }
}
