// fmt::Debug is not customizable generally we prefer maually implementing fmt::Display

// import fmt module to make it available
use std::fmt;

#[derive(Debug)]
struct Project<'a> {
    name: &'a str,
    amount: u8,
}

// implement fmt::Display trait for type Project
impl<'a> fmt::Display for Project<'a> {
    // this trait requires us to implement the fmt method with its signature
    // it doesn't assume how things should be printed
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // f represents the strem and it returns fmt::Result which says if the op was successfull or not
        write!(f, "New project {} worth {}", self.name, self.amount)
    }
}

fn main() {
    let new_project = Project {
        name: "Kalbero",
        amount: 42,
    };

    println!("Debug standard: {:?}", new_project);
    println!("Display custom implementation: {}", new_project);
}
