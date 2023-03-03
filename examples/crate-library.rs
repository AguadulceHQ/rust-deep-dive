// crate is a compilation unit in Rust
// by default the compilation with rustc is a crate file
// if mod declarations are present code from the module is inserted before compilation is done

// to compile as library use rustc --crate-type=lib crate-library.rs
// by default libraries get prefixed with lib, we can override with --crate-name

pub fn public_function() {
    println!("Called crate-library public_function()");
}

fn private_function() {
    println!("Called crate-library private_function()");
}

pub fn indirect_access() {
    print!("Called crate-library indirect_access, that\n> ");
    private_function();
}
