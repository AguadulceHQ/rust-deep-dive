pub struct Post {
    // this stays private we don't want to expose it directly
    content: String,
}

// we moved the encoding of the state to the type of the struct
pub struct DraftPost {
    // this is private
    content: String,
}

impl Post {
    // this now returns a DraftPost instead of a Post object
    pub fn new() -> DraftPost {
        DraftPost {
            content: String::new(),
        }
    }

    pub fn content(&self) -> &str {
        &self.content
    }
}

impl DraftPost {
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    // take ownership of self (consuming DraftPost instance) and transforming into PendingReviewPost
    pub fn request_review(self) -> PendingReviewPost {
        PendingReviewPost {
            content: self.content,
        }

        // there is no content method so the compiler will not compile the program if we call it
    }
}

pub struct PendingReviewPost {
    content: String,
}

impl PendingReviewPost {
    // this is the only way to get content ouf ot a post we need to approve it
    pub fn approve(self) -> Post {
        Post {
            content: self.content,
        }
    }
}
