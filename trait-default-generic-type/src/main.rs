use std::ops::Add;
use std::ops::Sub;
#[derive(Debug, Copy, Clone, PartialEq)]

struct BudgetRange {
    low: i32,
    high: i32,
}

// we don't override the default type parameter but we could
impl Add for BudgetRange {
    // associated type named Output that determines the type returned from fn add
    type Output = BudgetRange;

    fn add(self, other: BudgetRange) -> BudgetRange {
        BudgetRange {
            low: self.low + other.low,
            high: self.high + other.high,
        }
    }
}

// overriding the default parameter type for the trait
struct Discount(i32);

impl Sub<Discount> for BudgetRange {
    type Output = BudgetRange;

    fn sub(self, other: Discount) -> BudgetRange {
        BudgetRange {
            low: self.low - other.0,
            high: self.high - other.0,
        }
    }
}

fn main() {
    assert_eq!(
        BudgetRange { low: 10, high: 100 } + BudgetRange { low: 5, high: 50 },
        BudgetRange { low: 15, high: 150 }
    );

    assert_eq!(
        BudgetRange { low: 10, high: 100 } - Discount(10),
        BudgetRange { low: 0, high: 90 }
    );
}
