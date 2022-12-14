#[derive(Debug)]
enum Task {
    Cons(String, Rc<Task>),
    Nil,
}

use crate::Task::{Cons, Nil};
use std::rc::Rc; // bring into the scope as it’s not part of the prelude

fn main() {
    let major_task = Rc::new(Cons(
        String::from("Upgrade Polkadot's version to latest"),
        Rc::new(Cons(
            String::from("Run cargo audit and ensure that all warnings are resolved"),
            Rc::new(Nil),
        )),
    ));
    println!(
        "count references to major_task after creating major_task = {}",
        Rc::strong_count(&major_task)
    );

    let higher_priority_task = Cons(
        String::from("Deploy parachain code"),
        Rc::clone(&major_task),
    ); // we clone by passing a reference to a
    println!(
        "count references to major_task after creating higher_priority_task = {}",
        Rc::strong_count(&major_task)
    );
    {
        let highest_priority_task = Cons(String::from("Reserve ParaID"), Rc::clone(&major_task));
        // we clone by passing a reference to a
        println!(
            "count references to major_task after creating highest_priority_task = {}",
            Rc::strong_count(&major_task)
        );

        println!(
            "An Rc smart pointer allows to have multiple owners of some data. The task lists is {:?}",
            highest_priority_task
        );
    }
    println!(
        "count references to major_task after highest priority task goes out of scope = {}",
        Rc::strong_count(&major_task)
    );
}
