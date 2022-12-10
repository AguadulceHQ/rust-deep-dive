// we are having two generic types to be able to manage integer and decimals as ratings
#[derive(Debug)]
struct Feedback<CR, DR> {
    communication_rating: CR,
    deliverables_rating: DR,
}

fn main() {
    println!("Generic types allow to avoid code duplication 🔝");

    let integers = vec![3, 2, 42, 3];

    println!("The max of the integers is {}", maximum(&integers));

    let decimals = vec![3.0, 2.0, 42.0, 3.0];

    println!("The max of the decimals is {}", maximum(&decimals));

    let feedback = Feedback {
        communication_rating: 4,
        deliverables_rating: 5.0,
    };

    println!("Feedback release from client is {:?}", feedback);
}

// we need PartialOrd trait so that order is supported for this generic type
fn maximum<T: std::cmp::PartialOrd>(numbers: &[T]) -> &T {
    let mut max = &numbers[0];

    for number in numbers {
        if number > max {
            max = number;
        }
    }

    max
}
