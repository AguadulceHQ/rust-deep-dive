use trait_oop::Draw;

// let's define our own component that we want to design on the canvas
struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

// we need to implement the Draw trait
impl Draw for SelectBox {
    fn draw(&self) {
        // code to actually draw a select box
        println!("I am a SelectBox {} {}", self.width, self.height);
    }
}

// let's bring in scope what we need from the library
use trait_oop::{Button, Screen};

fn main() {
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 42,
                height: 10,
                options: vec![
                    String::from("We are going to make it"),
                    String::from("I am quitting"),
                    String::from("This is magic"),
                ],
            }),
            Box::new(Button {
                width: 42,
                height: 42,
                label: String::from("Submit"),
            }),
        ],
    }; // done with the screen

    // the beauty of trait object is that screen will run through the components
    // and call the relevant draw method that anyone implement Draw needs to define!
    screen.run();
}
