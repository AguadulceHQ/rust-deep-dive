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
        println!("I am a button {} {}", self.width, self.height);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn draw_trait_is_implemented() {
        let screen = Screen {
            components: vec![Box::new(Button {
                width: 42,
                height: 42,
                label: String::from("Submit"),
            })],
        }; // done with the screen

        screen.run();
    }

    #[test]
    fn button_has_required_info() {
        let button = Box::new(Button {
            width: 42,
            height: 42,
            label: String::from("Submit"),
        });

        assert_eq!(42, button.width);
        assert_eq!(42, button.height);
        assert_eq!(String::from("Submit"), button.label);
    }
}
