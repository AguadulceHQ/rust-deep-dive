// size of a List type is going to be size of i32 + the space for a box smart pointer. The Nil variant stores no value so it needs less space than the Cons variant
#[derive(Debug)]
enum List {
    Task(String, Box<List>),
    Nil, // variant with the base case for recursion
}

use crate::List::{Nil, Task};

fn main() {
    // creating a cons list
    let list = Task(
        String::from("Task 1"),
        Box::new(Task(
            String::from("Task 1.1"),
            Box::new(Task(String::from("Task 1.1.1"), Box::new(Nil))),
        )),
    );

    println!("Task list 👉 {:?}", list);
}
