use traits::{Evaluation, Feedback, NPS};

fn main() {
    let feedback = Feedback { score: 4 };

    println!(
        "The feedback's human readable score is {}",
        feedback.score_to_string()
    );

    let nps = NPS { rating: 5 };

    println!(
        "The feedback's human readable score is {}",
        nps.score_to_string()
    );
}
