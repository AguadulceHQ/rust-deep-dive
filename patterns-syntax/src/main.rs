fn main() {
    // literal syntax
    let project = 42;

    match project {
        1 => println!("Highest priority project"),
        42 => println!("Secret project"), // take action against a particular concrete value
        _ => println!("Lower priority project"),
    }

    // named variables
    let project = Some(42);
    let project_new = 3;

    match project {
        // this block creates a new scope
        Some(50) => println!("A lower priority project"),
        Some(project_new) => println!("Matched the new project {project_new}"), // we get here because project in this scope is matching anything it's not 3
        _ => println!("Default case"),
    }

    println!("We print this and as you can imagine project_new is equal to {project_new} as we are in the outer scope");

    // multiple patterns
    let project = 42;

    match project {
        3 | 42 => println!("A high priority project"), // pattern or operator
        42 => println!("We don't reach here as match exits after the first match"),
        _ => println!("The catchall case"),
    }

    // match ranges of values
    let feedback = 3;

    match feedback {
        1..=3 => println!("Low feedback"),
        _ => println!("High feedback"),
    }

    let answer = 'l';

    match answer {
        'a'..='j' => println!("Wrong"),
        'k'..='z' => println!("Correct guessed right!"),
        _ => println!("Invalid value"),
    }
}
