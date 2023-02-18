// for in can be used to iterate through an Iterator
// the easiest way to create an iteartor is to use the range notation a..b
// this yields values from a (inclusive) to b (esclusive) in steps of one

fn main() {
    // n will be 1, 2, ..., 100
    for n in 1..101 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }

    // we could write the same including also b by using a..=b
    for n in 1..=100 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }
}
