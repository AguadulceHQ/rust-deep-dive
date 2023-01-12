struct Feedback {
    communication: i32,
    overall: i32,
}

enum ProjectStatus {
    Analysis,
    Quote { time: i32, amount: i32 },
    Acceptance(bool),
    Started(TeamAssigned),
}

enum TeamAssigned {
    TeamWithManager(i32, String, String),
    TeamWithNoManager(i32, String),
}

fn main() {
    // deconstructing structs
    let f1 = Feedback {
        communication: 3,
        overall: 4,
    };

    let Feedback {
        communication: c,
        overall: o,
    } = f1; // match variables c and o to the values of fields in f1 struct

    let Feedback {
        communication,
        overall,
    } = f1; // shorthand notation give the frequent use case

    assert_eq!(3, c,);
    assert_eq!(3, communication);
    assert_eq!(4, o);
    assert_eq!(4, overall);

    // deconstructing enums
    let new_project = ProjectStatus::Started(TeamAssigned::TeamWithManager(
        42,
        String::from("Luca"),
        String::from("Tibi"),
    ));

    match new_project {
        ProjectStatus::Analysis => {
            // we can just match the literal as ProjectStatus::Analysis value has no variables
            println!("The Analysis variant has no data to destructure");
        }
        ProjectStatus::Quote { time, amount } => {
            // list the fields with variables and break them apart with shorthand form so that we can use them
            println!("The project will take {time} time and will cost {amount}");
        }
        ProjectStatus::Acceptance(status) => {
            println!("The project has been accepted: {status}");
        }
        ProjectStatus::Started(TeamAssigned::TeamWithManager(project, owner, dev)) => {
            // the number of variables matches the number of elements in the variant
            println!("The project with id {project} has started. The responsible is {owner} and the developer is {dev}");
        }
        ProjectStatus::Started(TeamAssigned::TeamWithNoManager(project, dev)) => {
            // the number of variables matches the number of elements in the variant
            println!("The project with id {project} has started. The developer is {dev}");
        }
    }
}
