// generics are useful to create generalized types and functionalities
// they are useful to reduce code duplication
// generic imply an extra care in managing types properly

// type parameter are an example of generic
// you use angle brackets and upper camel case <Aaa, Bbb, ...>
// generic type parameters are flagged <T> as convention

// generic function
// fn foo<T>(arg: T) { ... }

// type A
struct A;

// Single is a concrete type and A is defined as above
struct Single(A);

// SingleGen is a generic type due to <T>
// T can be anything including the concrete type A
struct SingleGen<T>(T);

fn main() {
    // Single is concrete and explicitly takes A
    let _s = Single(A);

    // Create a variable _char of type "SingleGen<char>"
    // SingleGen here has a type parameter explicitly defined

    let _char: SingleGen<char> = SingleGen('a');

    // SingleGen can also have a type parameter implicitly specified
    let _t = SingleGen(A);
    let _i32 = SingleGen(42); // uses i32
}
