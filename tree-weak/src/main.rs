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

    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );
    {
        println!("We are getting into an inner scope here to test counting");
        // we crate a new Node (contained in Rc) with leaf as a children and no parent
        let branch = Rc::new(Node {
            value: 42,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        // update reference of parent add a Weak<Node> reference
        // we get RefCell<Weak<Node>> and downgrade gives us Weak<Node>
        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        // Rc<Node> in branch has sc 1 and wk 1 (leaf.parent points to branch with Weak<Node>)
        println!(
            "branch strong = {}, weak = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch),
        );

        // leaf has sc 2 because branch now has a clone of the Rc<Node> of leaf stored in branch.children, wc is 0
        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );
    }

    println!("We are now in the outer scope let's check counts, branch no longer exists as its sc was 0 so it has been dropped");
    // now we can access the parent from the children and thanks to Weak without creating reference cycles
    // wc is still 1 but because Rc<Node> has sc 0 the Node is dropped
    // left has sc of 1 because leaf is now the only reference to the Rc<Node>
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );
}
