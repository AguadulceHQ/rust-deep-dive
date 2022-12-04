use std::collections::HashMap;

fn main() {
    let mut numbers = Vec::new();
    numbers.push(3);
    numbers.push(4);
    numbers.push(5);

    println!("Current vector {:?}", numbers);

    println!("The median value is {:?}", median(&numbers[..]));
}

fn median(numbers: &[i32]) -> i32 {
    let length = numbers.len();
    if length % 2 == 0 {
        let mid = length / 2;

        return (numbers[mid] + numbers[mid - 1]) / 2;
    } else {
        return numbers[length / 2];
    }
}
