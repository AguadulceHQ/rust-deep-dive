// diverging functions don't return
// they are marked with ! which is an empty type
#![feature(never_type)]
fn foo() -> ! {
    panic!("This call will never return");
}

// this is different from () type because that has exactly one possible value
// this one instead is empty

fn some_fn() {
    () // this fn returns although there is no info in the return value
}

// this is useful because this type can be cast to any other one and used at places where an exact type is required e.g. in match branches
fn sum_odd_numbers(up_to: u32) -> u32 {
    let mut acc = 0;
    for i in 0..up_to {
        // notice that the return type of this match expression must be u32
        // because of the type of the addition variable
        let addition: u32 = match i % 2 == 1 {
            // the i variable is of type u32
            true => i,
            // continue doesn't return u32, but it's fine because it never returns
            // this is fine as it does not violate the type requirements of the match expression
            false => continue,
        };
        acc += addition;
    }
    acc
}

fn main() {
    let a: () = some_fn();
    println!("This function returns this is why you can see this line");

    println!(
        "Sum of odd numbers up to 9 (excluding): {}",
        sum_odd_numbers(9)
    );
    let x = foo();
    println!("This line will never get printed instead");
}
