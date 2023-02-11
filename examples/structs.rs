// 3 types of structs
// tuple structs -> tuples
// typical C structs
// unit structs, which are field-less used for Generics

// attribute to tell the compiler to hide unused code warnings
#![allow(dead_code)]

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

// unit struct
struct Unit;

// tuple struct
struct Pair(i32, f32);

// struct with two fields
struct Point {
    x: f32,
    y: f32,
}

// reuse structs as data types
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

fn main() {
    // create a struct with field init shorthand
    let name = String::from("Luca");
    let age = 42;
    let luca = Person { name, age };

    // print debug struct
    println!("{:?}", luca);

    // initialize a point
    let point: Point = Point { x: 10.3, y: 0.5 };

    // access fields
    println!("Coords ({}, {})", point.x, point.y);

    // create new point with update syntax to use field of our first point
    let bottom_right = Point { x: 5.2, ..point };

    println!("Second point ({}, {})", bottom_right.x, bottom_right.y);

    // destructure the point and bind to variables
    let Point {
        x: left_edge,
        y: top_edge,
    } = point;

    let _rectangle = Rectangle {
        // struct instantiation is an expression
        top_left: Point {
            x: left_edge,
            y: top_edge,
        },
        bottom_right: bottom_right,
    };

    // instantiate a unit struct
    let _unit = Unit;

    // instantiate a tuple struct
    let pair = Pair(1, 0.2);

    // access the fields of a tuple struct
    println!("Pair contains {:?} and {:?}", pair.0, pair.1);

    // destructure a tuple struct
    let Pair(integer, float) = pair;

    println!("Pair contains {:?} and {:?}", integer, float);
}