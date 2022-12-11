use traits::{Evaluation, Feedback, NPS};

fn main() {
    let feedback = Feedback { score: 4 };

    println!(
        "The feedback's human readable score is {}",
        feedback.score_to_string()
    );

    println!("The score classification is {}", feedback.classification());

    println!("{}", feedback.display());

    let nps = NPS { rating: 8 };

    println!(
        "The feedback's human readable rating is {}",
        nps.score_to_string()
    );

    println!("The rating classification is {}", nps.classification());
}
