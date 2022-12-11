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

    check_score(&nps);

    let new_feedback = returns_evaluationable();

    check_score(&new_feedback);
}

// we take in an item that implements the trait Evaluation so that we can call that behavior
fn check_score(score: &impl Evaluation) {
    println!("{}", score.display());
}

fn returns_evaluationable() -> impl Evaluation {
    Feedback { score: 2 }
}
