pub trait Draw {
    fn draw(&self);
}

// this struct holds our canvas with different types of components
pub struct Screen {
    // trait object for any type inside box that implements Draw trait
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    // calls draw method to draw the canvas
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

// let's define some component

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        // code to actually draw a button
    }
}
