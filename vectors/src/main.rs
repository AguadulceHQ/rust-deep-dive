fn main() {
    // this is immutable we can't push anything in here
    let _feedback: Vec<u32> = Vec::new();
    // leveraging a macro to initialize the vector
    let feedback = vec![5, 3, 2];
    println!(
        "Got a new feedback list coming from folks about the new project 👉 {}",
        feedback[1]
    );

    // we can redeclare as that's no longer referenced and has been freed
    let mut feedback = Vec::new();
    feedback.push(5);
    feedback.push(3);
    feedback.push(2);

    println!(
        "Got a new feedback from a mutable Vector coming from folks about the new project 👉 {}",
        feedback[1]
    );

    let first_feedback: &i32 = &feedback[0];

    println!(
        "Got a new feedback, this time getting it out with a reference 👉 {}",
        first_feedback
    );

    let first_feedback: Option<&i32> = feedback.get(0);
    match first_feedback {
        Some(first_feedback) => println!(
            "Got a new feedback, this time getting it out with the get method that returns an Option going with match 👉 {}",
            first_feedback
        ),
        None => println!("No feedback to print folks"),
    }

    let feedbacks = vec![5, 3, 2];
    println!("We can also list all the feedback in a loop with the for loop 🔁");
    for feedback in &feedbacks {
        println!("{}", feedback);
    }

    let mut feedbacks = vec![5, 3, 2];
    println!("We can also iterate and change their value with a mutable reference. Let's bump our self esteem a bit! 🔁");
    for feedback in &mut feedbacks {
        *feedback += 2;
        println!("{}", feedback);
    }

    #[derive(Debug)]
    enum Document {
        Number(u32),
        Text(String),
    }

    let documents = vec![
        Document::Number(42),
        Document::Text(String::from("Noneofyourbusiness")),
    ];

    println!("And we can keep looping like there is no tomorrow");

    for doc in &documents {
        println!("{:#?}", doc);
    }
}
