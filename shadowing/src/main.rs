fn main() {
    let counter = 0;

    println!("The initial value of the counter: {counter}");

    let counter = counter + 1;

    println!("The value of counter after the increment is: {counter} 👌");

    {
        let counter = counter + 1;
        println!("The value of counter in the inner scope after the increment is: {counter} 👌");
    }

    println!(
        "The value of counter in the outer scope despite the inner scope's increment is: {counter} 😱"
    );
}
