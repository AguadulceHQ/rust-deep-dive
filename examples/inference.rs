// type inference is an engine that looks on how a variable is used to infer its type

fn main() {
    // the compiler knows this is a u8
    let num = 5u8;

    // create an empty vector
    let mut vec = Vec::new();

    // the compiler knows that it's Vec<_> but not the exact type

    vec.push(num);
    // now the compiler can infer that the type is Vec<u8>

    println!("{:?}", vec);
}
