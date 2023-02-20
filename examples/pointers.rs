fn main() {
    // assigns a reference of type i32
    // "&" means that we are assigning a reference
    let reference = &42;

    match reference {
        // pattern matched against &val e.g. &i32
        // & is dropped as part of the pattern so val = the i32
        &val => println!("Destructured a reference: {:?}", val),
    }

    // we dereference to avoid & while matching the pattern
    match *reference {
        val => println!("Dereferenced a reference: {:?}", val),
    }

    let value = 42;
    // this modifies the assignment so that we now have a reference to the element
    let ref value = 42;

    // references can be also retrieved via ref and ref mut
    let mut mut_value = 43;

    // use ref kw to create a reference
    match value {
        ref r => println!(
            "We match the reference with the ref kw to get a value {:?}",
            r
        ),
    }

    // use ref mut
    match mut_value {
        ref mut m => {
            // this is a reference need to dereference it to do operations with it
            *m -= 1;
            println!("We are back to the answer to life 'mut_value' {:?}", m);
        }
    }
}
