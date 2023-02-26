// closures capture variables from enclosing scopes
// this has consequences, using a closure required generics

// F must be generic but must implement Fn trait
// this is for a closure that takes no inputs and returns nothing
// we are using this for printing something
fn apply<F>(f: F)
where
    F: FnOnce(),
{
    f();
}

// the compiler implicitly creates a new anonymous structure to store the captured variables inside
// implementing the functionality via one of the traits Fn, FnMut, Fnonce for this unknown type, which gets stored until called
// since this new type is unknown any usage in a fn will require generics
// however unbounded type parameter <T> would still be ambiguos and not allowed, this is why we need bounding by one of those three traits
// it's kind of a substitute to declare a type

fn main() {
    let x = 42;

    // this closure captures x into an anon type and implements FN for it and stores it into print
    let print = || println!("{}", x);

    apply(print);
}
