// bring into scope so that we can use args fn
// we bring parent module in as the fn is 2 level nested at least it’s clear it comes from a module
// otherwise you may mistake args for your own fn

use std::env;
use std::fs;

fn main() {
    // we specify that we want a vector of Strings as collect can return other types of collections too
    // Rust generally infers the type but with this method we need to specify as it can return multiple
    let args: Vec<String> = env::args().collect();

    let (query, file_path) = parse_config(&args);

    println!("Searching for {}", query);
    println!("In file {}", file_path);

    // returns std::io::Result<String> that contain file’s contents
    let file_contents = fs::read_to_string(file_path).expect("Not able to read the file");

    println!("Text is\n\n {}", file_contents);
}

fn parse_config(args: &[String]) -> (&str, &str) {
    let query = &args[1];
    let file_path = &args[2];

    // we return references to arguments to which main has ownership
    (query, file_path)
}
