// bring into scope so that we can use args fn
// we bring parent module in as the fn is 2 level nested at least it’s clear it comes from a module
// otherwise you may mistake args for your own fn

use std::env;
use std::fs;

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn new(args: &[String]) -> Config {
        // owned strings go into the new defined struct
        let query = args[1].clone();

        // generally clone is avoided in this case the tradeoff is worth
        let file_path = args[2].clone();

        // clone cannot take ownership but only borrow from main
        // we need to clone those field to have ownership
        Config { query, file_path }
    }
}

fn main() {
    // we specify that we want a vector of Strings as collect can return other types of collections too
    // Rust generally infers the type but with this method we need to specify as it can return multiple
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args);

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    // returns std::io::Result<String> that contain file’s contents
    let file_contents = fs::read_to_string(config.file_path).expect("Not able to read the file");

    println!("Text is\n\n {}", file_contents);
}
