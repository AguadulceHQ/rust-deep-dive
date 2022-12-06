// fmt::Display for a structure where elements must be handled sequentially is tricky
// write! generates a fmt::Result so we need to handle all possible results
// we need to leverage the ? operator

// import the fmt module
use std::fmt;

// define a struct with a Vec inside
struct Project(Vec<i32>);

impl fmt::Display for Project {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // extract value using tuple indexing
        let vec = &self.0;

        // tries to write! if it errors return the error otherwise continue
        write!(f, "[")?;

        // iterate over v in vec and enumerate the iteration in count
        for (count, v) in vec.iter().enumerate() {
            // for every element expect the first add a comma
            // use ? to return errors
            if count != 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}: {}", count, v)?;
        }
        write!(f, "]")
    }
}

fn main() {
    let v = Project(vec![34, 55, 67]);
    println!("{}", v);
}
