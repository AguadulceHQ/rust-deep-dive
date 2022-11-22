// variable bindings have a scope
// they are constrained to live in a block
// block is a collection of statements enclosed by {}

fn main() {
    // binding in the main function
    let long = 1;

    // this is a block/smaller scope compared to main
    {
        let short = 2;
        println!("I can use short inside the smaller scope {}", short);
    }

    println!("In main I can only use long {}", long);
    // we can't print short because it's out of scope

    // but if we shadow the older value the compiler is fine with us using this new one because it's in scope
    let short = 2;
    println!("We can use the new short binding {}", short);
}
