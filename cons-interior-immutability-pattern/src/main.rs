#[derive(Debug)]
enum Task {
    Cons(Rc<RefCell<String>>, Rc<Task>),
    Nil,
}

use crate::Task::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc; // bring into the scope as it’s not part of the prelude

fn main() {
    // the type is Rc<RefCell<String>>
    // we wrap the string we want to change in RefCell
    let task = Rc::new(RefCell::new(String::from(
        "This is the most important task tomorrow.",
    )));

    let task_list_one = Rc::new(Cons(Rc::clone(&task), Rc::new(Nil)));

    // we don’t transfer ownership, with clone from Rc we have multiple ownership of task
    let task_list_two = Cons(
        Rc::new(RefCell::new(String::from(
            "Upgrade Polkadot's version to latest",
        ))),
        Rc::clone(&task_list_one),
    );
    let task_list_three = Cons(
        Rc::new(RefCell::new(String::from(
            "Run cargo audit and ensure that all warnings are resolved",
        ))),
        Rc::clone(&task_list_one),
    );

    println!(
        "Task list one before the borrow_mut call: {:?}",
        task_list_one
    );

    println!(
        "Task list two before the borrow_mut call: {:?}",
        task_list_two
    );

    println!(
        "Task list three before the borrow_mut call: {:?}",
        task_list_three
    );

    // borrow_mut uses automatic dereferencing to deref Rc<T> to the inner RefCell<T> value
    // borrow_mut returns a RefMut<T> smart pointer that we deref to change the inner value

    *task.borrow_mut() = String::from("This is the most important task today.");

    println!(
        "Task list one after the borrow_mut call: {:?}",
        task_list_one
    );

    println!(
        "Task list two after the borrow_mut call: {:?}",
        task_list_two
    );

    println!(
        "Task list three after the borrow_mut call: {:?}",
        task_list_three
    );
}
