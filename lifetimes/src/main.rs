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
