use std::ops::Deref;

struct TaskBox<T>(T); // we want to hold any type so we use generic parameter

impl<T> TaskBox<T> {
    fn new(x: T) -> TaskBox<T> {
        TaskBox(x)
    }
}

// we are implementing the Deref trait so that the compiler knows how to do it
impl<T> Deref for TaskBox<T> {
    type Target = T; // this is an associated type (a way of declaring sort of a generic parameter)

    fn deref(&self) -> &Self::Target {
        &self.0 // returns a reference to the value we want to access with * operator
                // .0 accesses the first value in a tupe struct
    }
}

fn print_task(task: &str) {
    println!("Message from the box 📦. The task is: {task}");
}

fn main() {
    let x = 5;
    let y = &x; // reference

    println!("x is equal to y in fact 👉 {} = {}", x, *y);
    assert_eq!(5, x);
    assert_eq!(5, *y); // dereference

    let z = 5;
    let w = Box::new(z); // a Box that holds a reference to z

    println!("z is equal to w in fact 👉 {} = {}", z, *w);
    assert_eq!(5, z);
    assert_eq!(5, *w); // dereference

    println!("Let's create our own Box that implements the deref trait 🧠");

    let task = TaskBox::new(String::from("Upgrade version of Polkadot to latest"));
    print_task(&task); // reference to a TaskBox<String> value but because we have the Deref trait Rust can turn &TaskBox<String> into &String by calling deref
                       // deref is then called again because String returns a string slice, so it converts &String to &str to match print_task fn definition
}
