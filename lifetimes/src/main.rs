#[derive(Debug)]
struct Client<'a> {
    name: &'a str,
}

fn main() {
    let project_one = String::from("Kalbero");
    let project_two = String::from("Apple");

    let result = projects_order(project_one.as_str(), project_two.as_str());

    println!("The first project in our order should be {}", result);

    let priority = projects_reset_priority(project_one.as_str(), project_two.as_str());

    println!("The priority got changed now we will focus on {}", priority);

    let client_name = "David";

    // this is fine because client_name is still in scope
    let client = Client { name: client_name };

    println!("Our client name is {}", client.name);

    projects_statement(
        project_one.as_str(),
        project_two.as_str(),
        "The top priority project is going to be",
    );
}

fn projects_order<'a>(first_project: &'a str, second_project: &'a str) -> &'a str {
    if first_project.len() < second_project.len() {
        first_project
    } else {
        second_project
    }
}

// dummy example but here we don't care of the lifetime of second_project anyway we are not returning it
// so it can safely go out of scope
fn projects_reset_priority<'a>(first_project: &'a str, second_project: &str) -> &'a str {
    first_project
}

use std::fmt::Display;
//  a dummy example take two string slices with same lifetimes
// accept a message of generic type T with same 'a lifetime as the stirng slices
// make sure that the message implements the Display trait because we need to print it
fn projects_statement<'a, T>(first_project: &'a str, second_project: &'a str, message: T) -> &'a str
where
    T: Display,
{
    if first_project.len() < second_project.len() {
        println!("{} {}", message, first_project);
        return first_project;
    }
    println!("{} {}", message, second_project);
    second_project
}
