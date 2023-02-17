// Rust provides loop kw to create an infinite loop
// break can be used to exit a loop anytime, continue can be used to skip the rest of a specific iteration and move ahead with the next one

fn main() {
    let mut count = 0u32;

    println!("Let's count to ♾️");

    loop {
        count += 1;

        if count == 3 {
            println!("Three");

            // skip the rest of the iteration and move to the next cycle
        }

        println!("{}", count);

        if count == 5 {
            println!("OK, that's enough!");
            // exit loop
            break;
        }
    }

    // loops are useful to retry something until it succeeds
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2; // returns value from loop
        }
    };

    assert_eq!(result, 20);
}
