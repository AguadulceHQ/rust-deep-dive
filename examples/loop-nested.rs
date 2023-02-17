// it's possible to use break/continue outer loops when you have nested loops
// you can use a 'label to annotate the loop so that you can be clear on the behaviour

#![allow(unreachable_code)]

fn main() {
    'outer: loop {
        println!("This is the outer loop");
        'inner: loop {
            println!("Entered inner loop");

            // this would break only the inner loop
            // break;

            // this breaks out the outer loop all together
            break 'outer;
        }
        println!("This won't be reached ever");
    }

    println!("We are out of the outer loop");
}
