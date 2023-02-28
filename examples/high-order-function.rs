// HOF are fn that take one or more fn and or produce a more useful fn
// HOF and lazy itertor give Rust its functional aspect

fn is_odd(n: u32) -> bool {
    n % 2 == 1
}

fn main() {
    println!("Find the sum of all the squared numbers under 1000");
    let upper = 1000;

    // imperative approach
    // declare accumulator variable
    let mut acc = 0;
    // iterate 0, 1, 2..infinity
    for n in 0.. {
        // square num
        let n_squared = n * n;

        if n_squared >= upper {
            // break loop
            break;
        } else if is_odd(n_squared) {
            // accumulate value if it's odd
            acc += n_squared;
        }
    }
    println!("Imperative style: {}", acc);

    // functional approach
    let sum_of_squared_odd_numers: u32 = (0..)
        .map(|n| n * n) // for each number in the range square
        .take_while(|&n_squared| n_squared < upper) // move ahead until this condition is true
        .filter(|&n_squared| is_odd(n_squared)) // check if odd
        .sum(); // sum odds
    println!("Functional style: {}", sum_of_squared_odd_numers);
}
