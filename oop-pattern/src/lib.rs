pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    pub fn new() -> Post {
        Post {
            // set state to Some value that holds a Box
            // Box points to a new instance of the Draft struct so that a new Post always starts as draft
            // this field is private we can't create a post in any other state
            state: Some(Box::new(Draft {})),
            // set content to empty string
            content: String::new(),
        }
    }
    // take a mutable reference to self because we change the Post instance
    // call push_str on String and pass the text argument to be added to the saved content
    // this behaviour is not part of the state pattern

    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    // we pass the post instance as argument
    // we return the value that's returned from using the content method on the state value
    pub fn content(&self) -> &str {
        // as_ref returns a reference to the value inside the Option rather than ownership of the value
        // state is Option<Box<dyn State>> so Option<&Box<dny State>> is returned
        // without as_ref we can't move state out of the borrowed &self of the function parameter
        // unwrap() won't panic because Post ensure that state will always contain a Some value when those methods are done
        // we use deref coeretion when we call content fn on &Box<dyn State> so it gets called on the type that implements the State trait
        // we need to add content to the State trait so the logic goes there
        self.state.as_ref().unwrap().content(self)
    }

    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review())
        }
    }

    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve())
        }
    }
}

// behaviour shared by different post states

trait State {
    // take  ownership a mutable reference to self (actually a Box holding the type)
    // this method returns a new state but temporarily sets the state to None because Rust doesn't allow unpouplated fields in structs

    fn request_review(self: Box<Self>) -> Box<dyn State>;

    // new method for Post
    fn approve(self: Box<Self>) -> Box<dyn State>;

    // default implementation for any state sthat doesn't have one
    // only Published will be giving out content to public
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        ""
    }
}

// state objects definition

struct Draft {}

// all state objects must implement the State trait to share behaviour

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        // returns a new boxed instance of a new PendingReview struct which represents the post waiting for review
        Box::new(PendingReview {})
    }

    // nothing should happen here because a draft first needs to go to review before being approved
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

struct PendingReview {}

impl State for PendingReview {
    // when we return a post that is in PendingReview it shouldn't do any transformation
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
    // returns a new boxed instance of the Published struct that implements the State trait
    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {})
    }
}

struct Published {}

impl State for Published {
    // nothing to do in this state for us
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    // override default implementation of the trait
    // we need lifetime annotation as we are taking a reference to a post as argument and returning a reference to part of that post so the lifetime of the returned reference is related to the lifetime of the post argument
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
}
