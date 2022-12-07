#![allow(unused)]
use std::fs::File;
use std::io::{self, Read};

fn main() {
    let res = read_text_from_file();

    println!("{:?}", res);
}

fn read_text_from_file() -> Result<String, io::Error> {
    // Result<T, E> is returned with specific types
    // If returns successfully you get an Ok variant with String
    // If errors returns io::Error which is the error that can happen in the body
    let text_from_file = File::open("hello.txt");

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
