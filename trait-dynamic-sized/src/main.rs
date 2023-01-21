fn main() {
    // the compiler doesn't support that. These strings require different space in memory so they can't use the same type
    //    let s1: str = "Hello";
    //    let s2: str = "Aguadulce";

    // these work because this are slices pointer that have the reference to the actual string
    // and length to know until where in memory the string is stored
    // the length of a reference that holds a pointer + a number for length is known to the compiler as those are both fixed types
    let s1: &str = "Hello";
    let s2: &str = "Aguadulce";

    println!("{} {}", s1, s2);

    let integers = vec![3, 2, 42, 3];
    println!("The max of the integers is {}", maximum(&integers));

    println!(
        "I am using a Sized trait implemented by default with give_back fn {}",
        integers[0]
    );
}

// this is actually equivalent to <T: Sized> because the compiler automatically adds a bound on Sized on any generic function
fn give_back<T>(t: T) -> T {
    t
}

// if we don't know if T will be known at compile time
// fn generic_unsure<T: ?Sized>(t: &T) {}
// this is equivalent to <T: ?Sized, std::cmp etc.>
fn maximum<T: std::cmp::PartialOrd>(numbers: &[T]) -> &T {
    let mut max = &numbers[0];

    for number in numbers {
        if number > max {
            max = number;
        }
    }

    max
}
