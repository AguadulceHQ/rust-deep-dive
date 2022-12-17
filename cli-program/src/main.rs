// bring into scope so that we can use args fn
// we bring parent module in as the fn is 2 level nested at least it’s clear it comes from a module
// otherwise you may mistake args for your own fn

use std::env;
use std::fs;
use std::process;
struct Config {
    query: String,
    file_path: String,
}

impl Config {
    // name this build as generally new doesn't need to handle errors
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("No enough arguments provided");
        }
        // owned strings go into the new defined struct
        let query = args[1].clone();

        // generally clone is avoided in this case the tradeoff is worth
        let file_path = args[2].clone();

        // clone cannot take ownership but only borrow from main
        // we need to clone those field to have ownership
        Ok(Config { query, file_path })
    }
}

fn main() {
    // we specify that we want a vector of Strings as collect can return other types of collections too
    // Rust generally infers the type but with this method we need to specify as it can return multiple
    let args: Vec<String> = env::args().collect();

    // method of std library that allows to define a non panic error handling otherwise returns the inner value of OK
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Issues in the parameters submitted to the program: {err}");

        // closes the program immediately and returns the exit status code specified
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    run(config);
}

fn run(config: Config) {
    let file_contents = fs::read_to_string(config.file_path).expect("Not able to read the file");
    println!("Text is\n\n {}", file_contents);
}
