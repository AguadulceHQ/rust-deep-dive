use std::fs::File;
use std::io::ErrorKind;

fn main() {
    // a crash caused voluntarily
    // panic!("We are dying here but still...hello world! 💀");

    // an unwanted crash
    // let scores = vec![10, 100, 1000];
    // scores[42];

    let onboarding_file = File::open("onboarding.txt");

    let onboarding = match onboarding_file {
        Ok(file) => file,
        Err(error) => panic!("Cannot open the onboarding file: {:?}", error),
    };

    println!("{:?}", onboarding);

    let project_file = File::open("project.txt");

    let project = match project_file {
        Ok(file) => file,
        Err(error) => match error.kind() {
            // io:Error is a struct with method kind
            ErrorKind::NotFound => match File::create("project.txt") {
                // kind returns io::ErrorKind value. Create also could fail so we need another match
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        },
    };

    // a cleaner version with closures instead of match expressions
    let documentation = File::open("documentation.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("documentation.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });

    // helper methods for Result<T, E> to give the value back or panic
    let proposal = File::open("proposal.txt").unwrap(); // this will panic and prints the error message
    let another_proposal = File::open("another_proposal.txt")
        .expect("Proposal.txt should be included in this project"); // panics with a custom message
}
