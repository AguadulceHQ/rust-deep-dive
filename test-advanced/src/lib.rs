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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_creates_a_new_feedback() {
        let result = Feedback::new(3);
        assert_eq!(result.value, 3);
    }

    #[test]
    #[should_panic(expected = "within 1 - poor and 5 - excellent")]
    // we want to pass the test if we get the right panic/error
    fn it_returns_an_error_if_value_is_not_within_limits() {
        let result = Feedback::new(10);
    }

    #[test]
    // we can also manage error scenarios ourselves within the test if we don't want to panic
    fn dummy_example() -> Result<(), String> {
        if 40 + 2 == 42 {
            Ok(())
        } else {
            // we actually don't reach here if the condition fails the test fails
            Err(String::from(
                "We didn't get the meaning of life unfortunately",
            ))
        }
    }
}
