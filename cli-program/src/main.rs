// bring into scope so that we can use args fn
// we bring parent module in as the fn is 2 level nested at least it’s clear it comes from a module
// otherwise you may mistake args for your own fn

use std::env;
use std::process;

use cli_program::Config;

fn main() {
    // we specify that we want a vector of Strings as collect can return other types of collections too
    // Rust generally infers the type but with this method we need to specify as it can return multiple
    let args: Vec<String> = env::args().collect();

    // method of std library that allows to define a non panic error handling otherwise returns the inner value of OK
    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Issues in the parameters submitted to the program: {err}");

        // closes the program immediately and returns the exit status code specified
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    // we use if let because run doesn’t return a value to unwrap
    // we care only about detecting the error not about () in OK
    if let Err(e) = cli_program::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
