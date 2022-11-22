// type safety in ensured through static typing in Rust
// in most cases the compiler can infer the type from the context

fn main() {
    let number = 1u32;
    let logic = true;
    let unit = ();

    // number_copy gets assigned u32 as type
    let number_copy = number;

    println!("An integer {:?}", number_copy);

    // the compiler warns about unused variable bindings
    // these warnings can be silenced by prefixing the variable with _
    let _unused = 3u32;

    // we can also declare a variable and initialize later
    // use rarely so that you don't forget uninitialized variables around
    let binding;
    let not_bound: u32;

    {
        let x = 2;
        binding = x * x;
    }

    println!("I can use binding {}", binding);

    // but I can't print not_bound because it's not initialized
}
