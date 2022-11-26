fn main() {
    let original_string = String::from("Oh boy I am a new string, where the actual data is 😎");

    println!("I am main printing the original_string before it's borrowed (I will keep ownership of it) {}", original_string);

    let string_length = calculate_length_of_borrowed_strings(&original_string);

    println!("The fn got a reference to our original_string, borrowed the value and calculated its length. We can still access the original string because main was always the owner. The length is {} 💯", string_length);

    let mut original_modifiable_string = String::from("New string in main but I am mutable 🦎");

    // passing a mutable reference
    modify_borrowed_strings(&mut original_modifiable_string);

    println!(
        "We are back to main 🔙 and we see that fn was able to modify our original string 👉 \n {} 🤯",
        original_modifiable_string
    );

    // I can do this because both are normal reference with no borrowing involved
    let reference1 = &original_modifiable_string;
    let reference2 = &original_modifiable_string;

    println!("Here we demonstrate that after this statement we can declare a new mutable reference because the non mutable references are dropped \n {} \n {}", reference1, reference2);

    // if we access reference1 after this line we would have an error here because we can't have mutable and immutable ref at the same time
    let reference3 = &mut original_modifiable_string;

    println!("Wow the mutable reference after the immutable ones works because those have been dropped {} 🤯", reference3);

    let string_from_fn_no_dangling = return_value_otherwise_dangling();

    println!("We are 🔙 in main and accessing the value returned. If it was a pointer to the variable we woulnd't be able because no dangling references are allowed \n {} 🎩", string_from_fn_no_dangling);
    fn calculate_length_of_borrowed_strings(s: &String) -> usize {
        // s doesn't have ownership so the value of "s" is not dropped we never had ownership in the first place
        s.len()
    }

    fn modify_borrowed_strings(s: &mut String) {
        s.push_str(" I am a fn that modified the original string");
    }

    fn return_value_otherwise_dangling() -> String {
        let string_to_main_from_fn = String::from(" I am a string from fn getting returned to main otherwise the variable in main will err because dangling is not allowed 🔝");
        // we cannot have -> &String and return & because the variable we declare here would go out of scope and main won't be able to access it
        string_to_main_from_fn
    }
}
