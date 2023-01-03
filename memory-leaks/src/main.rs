use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

// the second element of the Cons variant is RefCell<Rc<List>>
#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    // this method allows us to access conveniently the second item in a Cons variant
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}

fn main() {
    // create a list that holds 3
    let a = Rc::new(Cons(3, RefCell::new(Rc::new(Nil))));

    println!("a initial rc sc count = {}", Rc::strong_count(&a));
    // this is Nil we didn't create a next item yet
    println!("a next item = {:?}", a.tail());

    // create a second list that holds 42 that points to a
    let b = Rc::new(Cons(42, RefCell::new(Rc::clone(&a))));

    println!(
        "a rc sc count after b creation as b's next time is a = {}",
        Rc::strong_count(&a)
    );
    println!("b initial rc sc count = {}", Rc::strong_count(&b));
    println!("b next item is a = {:?}", b.tail());

    // point the list a instead of Nil to b creating a reference cycle
    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("b rc sc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc sc count after changing a = {}", Rc::strong_count(&a));

    // Uncomment the next line to see that we have a cycle;
    // it will overflow the stack
    // println!("a next item = {:?}", a.tail());
}
