pub trait Evaluation {
    fn score_to_string(&self) -> String;
}

pub struct Feedback {
    pub score: u8,
}

impl Evaluation for Feedback {
    fn score_to_string(&self) -> String {
        match self.score {
            1 => "One".to_string(),
            2 => "Two".to_string(),
            3 => "Three".to_string(),
            4 => "Four".to_string(),
            5 => "Five".to_string(),
            _ => "NA".to_string(),
        }
    }
}

pub struct NPS {
    pub rating: u8,
}

impl Evaluation for NPS {
    fn score_to_string(&self) -> String {
        match self.rating {
            1 => "One".to_string(),
            2 => "Two".to_string(),
            3 => "Three".to_string(),
            4 => "Four".to_string(),
            5 => "Five".to_string(),
            6 => "Six".to_string(),
            7 => "Seven".to_string(),
            8 => "Eight".to_string(),
            9 => "Nine".to_string(),
            _ => "NA".to_string(),
        }
    }
}
