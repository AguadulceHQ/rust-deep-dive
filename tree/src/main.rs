use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
// a node owns its children
struct Node {
    value: i32,
    // we use Rc<Node> because we want variables to access each Node in the tree directly
    // we wrap RefCell<T> around Vec<Rc<Node>> because we want to change the relationship between nodes
    children: RefCell<Vec<Rc<Node>>>,
}

fn main() {
    // one leaf with no children and value 3
    let leaf = Rc::new(Node {
        value: 3,
        children: RefCell::new(vec![]),
    });

    // value 42 and leaf as one children
    let branch = Rc::new(Node {
        value: 42,
        // leaf has now two owners leaf and the branch
        // in children we have a RefCell that points to a Vector that contains leaf as a Node
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });
}
