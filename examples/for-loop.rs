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

    // for in construct can interact with an Iterator in several ways
    // by default it applies into_iter function to the collection but there are other ways too
    // iter borrows each element of the collection through each iteration, leaving the collection untouched and available to reuse after the loop

    let names = vec!["Luca", "Daniel"];

    for name in names.iter() {
        match name {
            &"Luca" => println!("Aye aye!"),
            _ => println!("Hello {}", name),
        }
    }

    println!("Names {:?}", names);

    // into_iter consumes the collection so that the exact data in each iteration is provided
    // after consumption the collection is no longer available because it's moved within the loop
    let new_names = vec!["Luca", "Daniel"];

    for name in new_names.into_iter() {
        match name {
            "Luca" => println!("Aye aye!"),
            _ => println!("Hello {}", name),
        }
    }
    // we cannot access names now
    // println!("Names {:?}", new_names);

    // iter_mut mutably borrows each element of the collection allowing it to be modified in place

    let mut names = vec!["Luca", "Daniel"];

    for name in names.iter_mut() {
        *name = match name {
            &mut "Luca" => "Aye aye!",
            _ => "Hello",
        }
    }

    println!("Names {:?}", names);
}
