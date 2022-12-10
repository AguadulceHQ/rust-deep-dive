// we are having two generic types to be able to manage integer and decimals as ratings
#[derive(Debug)]
struct Feedback<CR, DR> {
    communication_rating: CR,
    deliverables_rating: DR,
}

enum EventStatus<P> {
    Confirmed(P),
    NotConfirmed,
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

    let conference = EventStatus::Confirmed(42);

    // if conference can be destructured into a Confirmed EventStatus then bind the value it contains to people
    if let EventStatus::Confirmed(people) = conference {
        println!(
            "The event is confirmed and {} people will attend 🎉",
            people
        );
    }
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
