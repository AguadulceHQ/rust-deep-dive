use std::fmt;

// a supertrait is a way for defining a trait that depends on another trait
// similar to adding a trait bound to a trait
// we specify that the Display trait is needed for OutlinePrint to work
// OutlinePrint works only on types that implement Display
trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

// if a type doesn't have Display trait we can implement it so that we can then implement OutlinePrint
impl fmt::Display for BudgetRange {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Range is {} - {}", self.low, self.high)
    }
}

struct BudgetRange {
    low: i32,
    high: i32,
}

// implement OutlinePrint and use default behaviour
impl OutlinePrint for BudgetRange {}

fn main() {
    let budget = BudgetRange { low: 5, high: 10 };
    budget.outline_print();
}
