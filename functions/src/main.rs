fn main() {
    println!("We are in the main function, calling another function 🧠");

    called();
    called_with_input(42);
    called_with_multiple_inputs(1, 'M');
    called_as_statement(42);
    let result = called_as_expression(42);
    println!(
        "I am main and the called as expression function gave me {} 🤯",
        result
    );
}

fn called() {
    println!("I have been called by main 🫡");
}

fn called_with_input(x: u32) {
    println!(
        "I have been called by main with a parameter x. The argument passed has been {} 🫡",
        x
    );
}

fn called_with_multiple_inputs(x: u32, unit_label: char) {
    println!(
        "I have been called by main with multiple parameters. Your height is more than {}{} 🫡",
        x, unit_label
    );
}

fn called_as_statement(x: u32) {
    println!(
        "I have been called by main and I am a statement. I will not return x + 1 despite me calculating. 🫡"
    );
    let _ = x + 1; // let _ is used to bypass the warning from the compiler (not an error!)
}

fn called_as_expression(x: u32) -> u32 {
    println!(
        "I have been called by main as an expression. I will return x + 1 which is {} 🫡",
        x + 1
    );
    x + 1
}
