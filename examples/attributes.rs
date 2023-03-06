// metadata applied to some module, crate or item
// generally used for conditional compilation of code, set crate name, disable lints, enable compiler features etc.
// mark fn as unit tests, or part of a benchmark

// #![crate_wide_attribute]

// #[attribute(value)]

#[derive(Debug)]
struct Project {
    id: u32,
}

fn main() {
    let p = Project { id: 3 };
    println!("{:?}", p);
}

// attributes can take multiple values
// #[attribute(value, value2)]

// attribute that disables the dead_code lint
#[allow(dead_code)]
fn unused_function() {}
