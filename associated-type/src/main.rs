pub trait Iterator {
    // this is a placeholder
    type Item;
    // next returns the associated type
    fn next(&mut self) -> Option<Self::Item>;
}

struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        // --snip--
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

fn main() {
    let mut c = Counter::new();

    // use the Iterator that takes an Item in our case a u32
    c.next();

    println!("{}", c.count);

    c.next();

    println!("{}", c.count);
}
