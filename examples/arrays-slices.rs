// array collection of objects of the same type T stored in contigous memory their length is known at compile time
// slices are similar to arry but their length is not known at compile time
// a slice is a two word object: a pointer to the data and the 2nd word the length of the slice
// word size depends on the processor architecture e.g. 64 bits on x86-64 so the compiler knows how much space to reserve for the program that uses slices

use std::mem;

// borrow a slice
fn take_slice(slice: &[i32]) {
    println!("first element of the slice: {}", slice[0]);
    println!("the slice has {} elements", slice.len());
}

fn main() {
    // fixed size array (type signature could be inferred)
    let fixed: [i32; 5] = [1, 2, 3, 4, 5];

    // initialize all elements to same value
    let initialized: [i32; 500] = [0; 500];

    // index and count
    println!("First element is {}", fixed[0]);
    println!("Total number of elements is {}", fixed.len());

    // arrays are stack allocated
    println!("Array occupies {} bytes", mem::size_of_val(&fixed));

    // arrays can be borrowed as slices
    println!("borrow the whole array as slice");
    take_slice(&fixed);

    // we can have a slice of the array
    println!("borrow a section of the array as slice");
    take_slice(&fixed[1..4]);

    // empty slice
    let empty: [u32; 0] = [];
    assert_eq!(&empty, &[]);
    assert_eq!(&empty, &[][..]); // same but more verbose

    // arrays can be safely access using .get which returns an Option
    // for example here we try one element more
    for i in 0..fixed.len() + 1 {
        match fixed.get(i) {
            Some(num) => println!("Iteration {} got a valid value: {}", i, num),
            None => println!("Too far {}", i),
        }
    }
}
