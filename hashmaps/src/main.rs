fn main() {
    use std::collections::HashMap;
    let mut projects_assigned = HashMap::new();

    let first_project = String::from("Aguadulce");
    let second_project = String::from("Kalbero");
    projects_assigned.insert(first_project, 1);
    projects_assigned.insert(second_project, 1);

    println!(
        "We have created a new mapping through a HashMap 👉 {:?}",
        projects_assigned
    );

    let first_project = String::from("Aguadulce");
    let lead_for_aguadulce = projects_assigned.get(&first_project).copied().unwrap_or(0);

    println!(
        "ID of team member lead for the Aguadulce project {} 🙇",
        lead_for_aguadulce
    );

    println!("Here is a list of all the projects we are working on 🎩");

    for (key, value) in &projects_assigned {
        println!("Project {} assigned to {}", key, value);
    }

    projects_assigned.insert(String::from("Qualiv"), 2);

    println!("Here is an updated list of all the projects we are working on 🎩");

    println!("{:?}", projects_assigned);

    println!("We can also check if a project has been already added and avoid duplication ❗");

    projects_assigned.entry(String::from("Qualiv")).or_insert(2);

    println!("{:?}", projects_assigned);

    println!("We may be interested in knowing how many projects are assigned to one person 🧠");

    let mut projects_per_member = HashMap::new();

    for (project, member) in &projects_assigned {
        // if new add entry to hash map
        let count = projects_per_member.entry(member).or_insert(0);
        *count += 1;
    }

    for (key, value) in &projects_per_member {
        println!("Team member #{} has assigned {} projects", key, value);
    }
}
