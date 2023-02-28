// Iterator::find is a fn that iterators over an Iterator and searches for the first value that satisfies some condition
// if non of the elements satisfies the condition it returns None

pub trait Iterator {
    // type being iterated over
    type Item;

    // find takes &mut self meaning the caller may be borrowed and modified but not consumed
    fn find<P>(&mut self, predicate: P) -> Option<Self::Item>
    where
        // FnMut meaning that any captured variable may at most be modified not consumed
        // &Self::Item states it takes arguments to the closure by reference
        P: FnMut(&Self::Item) -> bool;
}

fn main() {
    let vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];

    // iter() yields &i32
    let mut iter = vec1.iter();
    // into_iter() for vec yields i32
    let mut into_iter = vec2.into_iter();

    // we want to reference one of its items so we need to destructure &&i32 to i32
    println!("Find 2 in vec1: {:?}", iter.find(|&&x| x == 2));
    // we want to reference one of its items so we destructure &i32 to i32
    println!("Find 2 in vec2: {:?}", into_iter.find(|&x| x == 2));

    let array1 = [1, 2, 3];
    let array2 = [3, 2, 5, 6];

    // iter() for arrays yields &i32
    println!("Find 2 in array1: {:?}", array1.iter().find(|&&x| x == 2));
    println!(
        "Find 2 in array2: {:?}",
        array2.into_iter().find(|&x| *x == 2)
    );

    let vec = vec![1, 9, 3, 3, 13, 2];

    // iter() yields &i32 and position() does not take a reference, we need to destructure &i32 to i32 to get the index
    let index_of_first_even_number = vec.iter().position(|&x| x % 2 == 0);
    assert_eq!(index_of_first_even_number, Some(5));

    // into_iter() yields i32, so we don't need to destructure
    let index_of_first_negative_number = vec.into_iter().position(|x| x < 0);
    assert_eq!(index_of_first_negative_number, None);
}
