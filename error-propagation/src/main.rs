#![allow(unused)]
use std::fs::File;
use std::io::{self, Read};

fn main() {
    let res = read_text_from_file();

    println!("{:?}", res);

    let res_operator = read_text_from_file_with_operator();

    println!("{:?}", res_operator);

    let res_optimized = read_text_from_file_optimized();

    println!("{:?}", res_optimized);
}

fn read_text_from_file() -> Result<String, io::Error> {
    // Result<T, E> is returned with specific types
    // If returns successfully you get an Ok variant with String
    // If errors returns io::Error which is the error that can happen in the body
    let text_from_file = File::open("brief.txt");

    let mut text = match text_from_file {
        Ok(file) => file,
        Err(e) => return Err(e), // returns the error to the caller
    };

    let mut text_found = String::new();

    match text.read_to_string(&mut text_found) {
        // this also may fail
        Ok(_) => Ok(text_found),
        Err(e) => Err(e), // here also we return the error to the caller but because it’s the last expression we don’t use the kw
    }
}

fn read_text_from_file_with_operator() -> Result<String, io::Error> {
    // we introduce the ? operator
    let mut text_from_file = File::open("brief.txt")?;
    // a variable where we hold what we'll read
    let mut text_found = String::new();
    // a common operation of reading from file into a string that we reference
    // text_from_file needs to be a mutable
    text_from_file.read_to_string(&mut text_found)?;
    // return the value if success otherwise propagate the error
    Ok(text_found)
}

fn read_text_from_file_optimized() -> Result<String, io::Error> {
    let mut text_found = String::new();
    // we immediately pass the file instance and call the method on it
    // ? will propagate an error in case the file doesn't exist
    File::open("project.txt")?.read_to_string(&mut text_found)?;
    // return the value if success otherwise propagate the error
    Ok(text_found)
}
