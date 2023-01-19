use std::fmt;

// newtype pattern
struct Wrapper(Vec<String>);

// we could not implement the Display trait directly on Vec<T> as that type is not defined in this crate
impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // self.0 accesses the inner Vec<String> because Wrapper is a tuple struct and Vec<String> is the tem at index 0 in the tuple
        write!(f, "[{}]", self.0.join(", "))
    }
}
fn main() {
    let wrapper = Wrapper(vec![String::from("Aleph"), String::from("Kalbero")]);
    println!("Our vector wrapped in the newtype pattern is {}", wrapper);
}
