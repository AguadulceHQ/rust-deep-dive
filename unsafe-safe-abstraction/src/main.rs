fn main() {
    let mut numbers = vec![1, 2, 3, 4, 5];

    let r = &mut numbers[..];

    // we use an unsafe fn from std
    let (first_half, second_half) = r.split_at_mut(3);

    assert_eq!(first_half, &mut [1, 2, 3]);
    assert_eq!(second_half, &mut [4, 5]);

    let (third_half, fourth_half) = split_at_mut_equivalent(r, 3);

    assert_eq!(third_half, &mut [1, 2, 3]);
    assert_eq!(fourth_half, &mut [4, 5]);
}

use std::slice;

// this is a safe abstraction within which we use isolated unsafe code
fn split_at_mut_equivalent(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    // slices are pointers to some data + the length of it
    let length = values.len();
    let pointer = values.as_mut_ptr();

    assert!(mid <= length);

    unsafe {
        (
            // this fn is unsafe because it assumes that the raw pointer is a valid reference
            // takes a raw pointer and the length and creates a slice
            slice::from_raw_parts_mut(pointer, mid),
            slice::from_raw_parts_mut(pointer.add(mid), length - mid),
        )
    }
}
