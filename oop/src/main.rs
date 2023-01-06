pub struct AveragedFeedback {
    feedback: Vec<i32>,
    average: f64,
}

impl AveragedFeedback {
    pub fn add(&mut self, value: i32) {
        self.feedback.push(value);
        self.update_average();
    }

    pub fn remove(&mut self) -> Option<i32> {
        let removed = self.feedback.pop();
        match removed {
            Some(value) => {
                self.update_average();
                Some(value)
            }
            None => None,
        }
    }

    pub fn average(&self) -> f64 {
        self.average
    }

    fn update_average(&mut self) {
        let total: i32 = self.feedback.iter().sum();
        self.average = total as f64 / self.feedback.len() as f64;
    }
}

fn main() {
    let mut feedback = AveragedFeedback {
        feedback: vec![],
        average: 0.0,
    };

    feedback.add(4);
    feedback.add(4);
    feedback.add(2);
    println!("The average feedback is {}", feedback.average());
}
