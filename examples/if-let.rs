// for some use cases match is not the best

fn main() {
    let optional = Some(7);

    match optional {
        // we need two levels of indentation to get i
        Some(i) => {
            println!("We got {:?}", i);
        }
        // we need to handle all cases, waste space
        _ => {}
    }

    // we can instead use the if let construct
    let number = Some(7);
    let emoji: Option<i32> = None;

    if let Some(i) = number {
        println!("Matched {:?}", i);
    }

    // to provide another condition to test
    let another = false;

    if let Some(i) = emoji {
        println!("Matched {:?}", i);
    } else if another {
        println!("Didn't match a number but the other condition");
    } else {
        println!("Didn't match a number and not even the other condition");
    }

    // we can also destructure enums

    // Our example enum
    enum Foo {
        Bar,
        Baz,
        Qux(u32),
    }

    enum Bar {
        Foo,
    }

    let c = Foo::Qux(100);

    // Variable c matches Foo::Qux which has a value
    if let Foo::Qux(value) = c {
        println!("c is {}", value);
    }

    // Binding also works with `if let`
    if let Foo::Qux(value @ 100) = c {
        println!("c is one hundred");
    }

    // now a more complex example
    // we can match non parameterized enum variants that don't implement/derive PartialEq
    let a = Bar::Foo;

    // here if Foo::Bar == a would fail because PartialEq trait is not implemented
    if let Bar::Foo = a {
        println!("Matched a Barfoo");
    }
}
