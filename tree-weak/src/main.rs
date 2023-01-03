use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
struct Node {
    value: i32,
    // a children can refer to its parent but doesn’t own it
    // we use a Weak smart pointer to achieve that as it has different reference counting compared to Rc
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

fn main() {
    // create a Node with an empty parent and an empty vector of children
    let leaf = Rc::new(Node {
        value: 3,
        // starts without a parent initially gets assigned later
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    // the parent is None
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

    // we crate a new Node (contained in Rc) with leaf as a children and no parent
    let branch = Rc::new(Node {
        value: 42,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });

    // update reference of parent add a Weak<Node> reference
    // we get RefCell<Weak<Node>> and downgrade gives us Weak<Node>
    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

    // now we can access the parent from the children and thanks to Weak without creating reference cycles
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
}
