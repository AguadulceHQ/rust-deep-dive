//! # Cli Program Doc
//!
//! `cli_program_doc` is an utility that allows you to search a string into a text

use std::env;

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    // env::args passed from main is defined in std::env::Args as implementing the Iterator trait and returning String values
    // the args parameter has a generic type with trait bounds Iterator<Item = String> instead of &[String]
    // we are taking ownership of argos and we’ll be mutating it by iterating over it so we need mut
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        // the first element will be the name of the program, ignore it
        args.next();

        // thanks to the iterator we don't need to clone the values
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        // iterate and use match to extract the value
        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };

        // env::var returns a Result that returns Err variant if the env variable is not set
        // is_ok returns false if the variable is not set, so we perform a case sensitive search
        // we use is_ok instead of unwrap/expect because we don’t need the value just know if it’s set or not

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        // clone cannot take ownership but only borrow from main
        // we need to clone those field to have ownership
        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

use std::error::Error;
use std::fs;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    //// we use the ? operator in place of expect, instead of panicking ? will return the error value from the current function to the caller to handle
    let file_contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_non_sensitive(&config.query, &file_contents)
    } else {
        search(&config.query, &file_contents)
    }; // this is an assignment

    for line in results {
        println!("{line}");
    }

    Ok(())
}

/// Does a case sensitive search of the query you pass in the contents and returns matching lines
/// ```
/// let query = "Guest";
/// let contents = "\
///       This is a Guest line.
///       Rust is safe, fast, productive.";
///
/// assert_eq!(vec!["This is a Guest line."], cli_program_doc::search(query, contents));
/// ```

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // by using an iterator and a closure we avoid using a temporary vector that handles state
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_non_sensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();

    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
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
