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
}
