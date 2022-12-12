fn main() {
    let project_one = String::from("Kalbero");
    let project_two = String::from("Apple");

    let result = projects_order(project_one.as_str(), project_two.as_str());

    println!("The first project in our order should be {}", result);
}

fn projects_order<'a>(first_project: &'a str, second_project: &'a str) -> &'a str {
    if first_project.len() < second_project.len() {
        first_project
    } else {
        second_project
    }
}
