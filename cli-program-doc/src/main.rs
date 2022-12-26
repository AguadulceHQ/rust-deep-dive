// bring into scope so that we can use args fn
// we bring parent module in as the fn is 2 level nested at least it’s clear it comes from a module
// otherwise you may mistake args for your own fn

use std::env;
use std::process;

use cli_program_doc::Config;

fn main() {
    // env::args() returns an Iterator and we pass that to build
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Issues in the parameters submitted to the program: {err}");

        // closes the program immediately and returns the exit status code specified
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    // we use if let because run doesn’t return a value to unwrap
    // we care only about detecting the error not about () in OK
    if let Err(e) = cli_program_doc::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
