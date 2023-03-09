// a type T becomes a generic in a fn when preceded by <T>

// sometimes generic fn parameters require explicit type specification
// when the compiler can't infer the necessary type parameters

struct A; // concrete type A
struct S(A); // concrete type S
struct SGen<T>(T); // generic type SGen

// all fn take ownership of the variable passed and immediately go out scope, freeing the variable

// define fn reg_fn takes an argument _s of type S
// this is not a generic fn

fn reg_fn(_s: S) {}

// explicitly type parameter A, but because A has not been specified as a generic type parameter it is not a generic
fn gen_spec_t(_s: SGen<A>) {}

// this fn is not a generic, the concrete type is i32
fn gen_spec_i32(_s: SGen<i32>) {}

// takes argument _s of type SGen<T>
// because it's preceded by <T> this fn is generic over T
fn generic<T>(_s: SGen<T>) {}

fn main() {
    reg_fn(S(A));
    gen_spec_t(SGen(A)); // implicitly defined type parameter A
    gen_spec_i32(SGen(42)); // implicitly specified type parameter i32

    // explicitly specified type parameter char to generic()
    generic::<char>(SGen('a'));

    // implicitly specified type parameter char to generic()
    generic(SGen('c'));
}
