pub struct Config {
    pub query: String,
    pub file_path: String,
}

impl Config {
    // name this build as generally new doesn't need to handle errors
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
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

use std::error::Error;
use std::fs;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    //// we use the ? operator in place of expect, instead of panicking ? will return the error value from the current function to the caller to handle
    let file_contents = fs::read_to_string(config.file_path)?;
    println!("Text is\n\n {}", file_contents);
    Ok(())
}
