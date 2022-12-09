#[derive(Debug)]
pub struct Feedback {
    // struct with the number that will be stored
    value: i8,
}

// define behaviour of this enum type
impl Feedback {
    pub fn new(value: i8) -> Feedback {
        if value < 1 || value > 5 {
            panic!(
                "Feedback must be within 1 - poor and 5 - excellent. You submitted {}",
                value
            );
        }
        Feedback { value } // returns value if no panic occurred
    }

    // borrows self and returns the i8 value
    // we are exposting a getter because by default struct fields are private
    pub fn value(&self) -> i8 {
        self.value
    }
}

fn main() {
    let feedback = Feedback::new(3);
    println!("Valid feedback {:?} ✅", feedback);

    println!("Thanks to Rust's type safety we can be sure that if an improper critical value is passed we crash safely without adding tons of validations 🔝");
    let invalid_feedback = Feedback::new(42);
}
