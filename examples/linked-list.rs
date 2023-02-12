// implementation of a Linked List with Enums

use crate::List::*;

enum List {
    // Cons: a tuple struct that wraps an element and a pointer to the next node
    Cons(u32, Box<List>),
    // Nil is a node that signifies the end of the list
    Nil,
}

// methods can be attached to the enum type

impl List {
    // create an empty List
    fn new() -> List {
        Nil
    }

    // takes a list and returns the same list with a new element in front
    fn prepend(self, elem: u32) -> List {
        // Cons also has type List
        Cons(elem, Box::new(self))
    }

    // length of the list
    fn len(&self) -> u32 {
        // self has to be matched because the behaviour depends on the variant of self
        // self has type &List and *self is List
        // generally we prefer to match a concrete type T instead of the reference &T
        match *self {
            // can't take ownership because self is borrowed
            // instead take a reference to the tail
            Cons(_, ref tail) => 1 + tail.len(),
            // Base case for recursion when the list has 0 length
            Nil => 0,
        }
    }

    // return a heap allocated string representation of the list
    fn stringify(&self) -> String {
        match *self {
            Cons(head, ref tail) => {
                // format is similar to print but returns a heap allocated string instead of printing to the console
                format!("{}, {}", head, tail.stringify())
            }
            Nil => {
                format!("Nil")
            }
        }
    }
}

fn main() {
    // Create empty linked list
    let mut list = List::new();

    // prepend
    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);

    // show final state
    println!("Linked list has length {}", list.len());
    println!("The list is {}", list.stringify());
}
