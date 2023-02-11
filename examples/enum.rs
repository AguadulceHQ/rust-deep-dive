// enum kw creates a type which may be one of a few variants
// any variant valid as a struct is a valid enum

enum WebEvent {
    PageLoad,
    PageUnload,
    KeyPress(char),
    Paste(String),
    Click { x: i64, y: i64 },
}

fn inspect(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("Page loaded"),
        WebEvent::PageUnload => println!("Page unloaded"),
        // destructure c from inside the enum
        WebEvent::KeyPress(c) => println!("Pressed {}", c),
        WebEvent::Paste(s) => println!("Pasted {}", s),
        // destructure Click into x and y
        WebEvent::Click { x, y } => {
            println!("Clicked at x={}, y={}", x, y);
        }
    }
}

// type alias are useful if an enum is too long or generic and we want to rename it

enum VeryLongNameThatWeWontType {
    Add,
    Subtract,
}

// create a type alias
type Operations = VeryLongNameThatWeWontType;

// very useful also here
impl Operations {
    fn run(&self, x: i32, y: i32) -> i32 {
        match self {
            // self is a type alias under the hood
            Self::Add => x + y,
            Self::Subtract => x - y,
        }
    }
}

// two C like enums

// implicit discriminator for this enum starts at 0
enum Number {
    Zero,
    One,
    Two,
}

// enum with explicit discriminator
enum Color {
    Red = 0xff0000,
    Green = 0x00ff00,
    Blue = 0x0000ff,
}

fn main() {
    let pressed = WebEvent::KeyPress('x');
    // to_owned creates an owned String from a string slice
    let pasted = WebEvent::Paste("My text".to_owned());
    let click = WebEvent::Click { x: 20, y: 42 };
    let load = WebEvent::PageLoad;
    let unload = WebEvent::PageUnload;

    inspect(pressed);
    inspect(pasted);
    inspect(click);
    inspect(load);
    inspect(unload);

    // we can refer to each variant with its alias
    let x = Operations::Add;
    println!("2 plus 3 is {}", x.run(2, 3));

    // enums can be cast as integers
    println!("zero is {}", Number::Zero as i32); // this would cast to 0 also if it was say "Bla"
    println!("roses are #{:06x}", Color::Red as i32);
}
