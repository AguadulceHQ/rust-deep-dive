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

    let x: i32 = -1;

    println!("The value of x: i32 in the outer scope is: {x}");

    {
        let x: u32 = 1;
        println!("The value of x: u32 in the inner scope is: {x}");
        println!("We changed the type of a variable using the same name, that's not possible with mut 🤯");
    }
}
