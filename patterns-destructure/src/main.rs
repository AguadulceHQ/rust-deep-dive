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

    let new_project_two = ProjectStatus::Started(TeamAssigned::TeamWithManager(
        42,
        String::from("Luca"),
        String::from("Tibi"),
    ));

    if let ProjectStatus::Started(TeamAssigned::TeamWithManager(project, manager, dev)) =
        new_project_two
    {
        team(project, manager, dev);
    }

    let projects = (42, 3, 84, 5);

    match projects {
        // ignore pattern unused variable
        (first, _second, third, _) => {
            println!("Priority projects are {first} and {third}");
            println!("Also {_second} is important given that we bound it with the prefix");
        }
    }

    // ignore pattern for remaining parts of a value
    let new_projects = (42, 3, 5, 84);

    match new_projects {
        (first, .., last) => {
            println!("Important projects are {first} and {last}")
        }
    }

    let new_feedback = Some(Feedback {
        communication: 3,
        overall: 4,
    });

    // match guard syntax pattern
    match new_feedback {
        Some(f) if f.overall < 3 => println!(
            "When doing a thorough review this feedback should be evaluated carefully as it is {}",
            f.overall
        ),
        // because we are using a match guard the compiler won't check exhaustiveness for us
        Some(f) => println!("This is a regular feedback, skip in the review"),
        None => (),
    }

    let threshold = 3;
    let new_feedback_two = Feedback {
        communication: 3,
        overall: 4,
    };

    // or operator in match guard
    match threshold {
        1 | 2 | 3 if new_feedback_two.communication < threshold => println!(
            "When doing a thorough review this feedback should be evaluated carefully as it is {}",
            new_feedback_two.communication
        ),
        // because we are using a match guard the compiler won't check exhaustiveness for us
        _ => println!("This is a regular feedback, skip in the review"),
    }

    // @ operator

    let new_project = ProjectStatus::Quote {
        time: 32,
        amount: 500,
    };

    match new_project {
        ProjectStatus::Quote {
            // by using id_variable @ we capture whatever value matched the range while also testing that the value matched the range pattern
            amount: price @ 0..=1000,
            time: t,
        } => println!("This is a small range project: {}$", price),
        _ => println!("Nothing to be done for this project"),
    }
}

// ignore pattern for function parameter
fn team(_: i32, manager: String, dev: String) {
    println!("The team is composed by {manager} and {dev}");
}
