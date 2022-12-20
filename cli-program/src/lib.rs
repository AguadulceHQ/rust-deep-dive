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
    for line in search(&config.query, &file_contents) {
        println!("{line}");
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

pub fn search_non_sensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod tests {
    // bring into scope the code in the rest of the library
    // we use the glob operator to do so
    use super::*;

    #[test]
    fn search_kw_in_text() {
        let query = "safe";
        // the backlash tells Rust not to put a newline char at the beginning of the contents of this string literal
        let contents = "\
        Rust:
        safe, fast, productive.
        Pick three.
        Safe, fast, productive.";
        // check that we return the line that matches our query in that content
        assert_eq!(
            vec!["        safe, fast, productive."],
            search(query, contents)
        );
    }

    #[test]
    fn search_kw_in_text_non_sensitive() {
        let query = "RUSt";
        let contents = "\
        Rust:
        safe, fast, productive.
        Pick three.
        Safe, fast, productive.";

        // Rust: should be the line returned if there is a match
        assert_eq!(vec!["Rust:"], search_non_sensitive(query, contents));
    }
}
