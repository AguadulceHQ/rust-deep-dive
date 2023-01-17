use std::ops::Add;

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

fn main() {
    assert_eq!(
        BudgetRange { low: 10, high: 100 } + BudgetRange { low: 5, high: 50 },
        BudgetRange { low: 15, high: 150 }
    )
}
