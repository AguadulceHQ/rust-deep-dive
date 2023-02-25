// closures can capture variables by reference &T, by mutable reference &mut T, by value T
// you start by reference generally and go lower in the hierarchy only when required

fn main() {
    use std::mem;

    let color = String::from("green");

    // closure stored in print immediately borrows & color and stores the borrow within the closure
    // it will stay borrowed until print is used the last time
    // println required an immutable reference so this is by reference
    let print = || println!("color {}", color);

    // call the closure that uses borrow
    print();

    // color can be borrowed again immutably
    let _borrow_again = &color;
    print();

    // we can move or reborrow only after a final use of print
    let _color_moved = color;

    let mut count = 0;

    // closure to increment count can take either &mut count or count
    // &mut count is less restrictive

    // mut is required on inc because &mut is stored inside
    // calling the closure mutates the closure so mut is required
    let mut inc = || {
        count += 1;
        println!("Count {}", count);
    };

    // call the closure using a mutable borrow
    inc();

    // we can't reborrow here because count is called again later
    // let borrowagain = &count;
    inc();

    // the closure no longer needs to borrow &mut count
    // so now we can borrow again
    let _count_borrow_again = &mut count;

    // this type doesn't implement the Copy trait
    let movable = Box::new(42);

    // mem::drop requires T so take by value
    // a copy type would copy into the closure leaving the original untouched
    // a non copy instead must move immediately into the closure
    let consume = || {
        println!("movable {:?}", movable);
        mem::drop(movable);
    };

    // consume consumes the variables so this can be called only once
    consume();
}
