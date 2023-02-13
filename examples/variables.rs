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
}
