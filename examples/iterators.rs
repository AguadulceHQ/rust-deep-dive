// Iterator::any is a fn that when passed iterator returns true if any element satisfies the predicate otherwise it returns false

pub trait Iterator {
    // the type of the item iteratred over
    type Item;

    // any takes &mut self, the caller may be borrowed and modified but not consumed
    fn any<F>(&mut self, f: F) -> bool
    where
        // FnMut any captured variable may at most be modified but not consumed
        // Self::Item states it takes arguments to the closure by value
        F: FnMut(Self::Item) -> bool;
}

fn main() {
    let vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];

    // iter() for vecs yields &i32. Destructure to i32
    println!("2 in vec1: {}", vec1.iter().any(|&x| x == 2));
    // into_iter() for vecs yields i32. No destructuring required
    println!("2 in vec2: {}", vec2.into_iter().any(|x| x == 2));

    // iter() only borrows vec1 and its elements so it can be used again
    println!("vec1 len: {}", vec1.len());
    println!("First element of vec1 is: {}", vec1[0]);

    let array1 = [1, 2, 3];
    let array2 = [4, 5, 6];

    // iter() for arrays yields &i32
    println!("2 in array1: {}", array1.iter().any(|&x| x == 2));
    // into_iter() for arrays yields &i32
    println!("2 in array2: {}", array2.into_iter().any(|&x| x == 2));
}
