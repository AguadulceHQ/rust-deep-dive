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
#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

// reuse structs as data types
#[derive(Debug)]
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

fn rect_area(rect: &Rectangle) -> f32 {
    let &Rectangle {
        top_left: Point { x: x1, y: y1 },
        bottom_right: Point { x: x2, y: y2 },
    } = rect;
    return (x2 - x1).abs() * (y2 - y1).abs();
}

fn square(origin: Point, dimension: f32) -> Rectangle {
    // bring x, y in the scope from origin
    // let binding those values to x, y
    // destructuring Point { . } it's just to inform how to do that
    let Point { x, y } = origin;

    Rectangle {
        top_left: origin,
        bottom_right: Point {
            x: x + dimension,
            y: y + dimension,
        },
    }
}

// struct defined for destructuring example
struct ThreePoint {
    xy: (u32, u32),
    z: u32,
}

fn main() {
    // create a struct with field init shorthand
    let name = String::from("Luca");
    let age = 42;
    let luca = Person { name, age };

    // print debug struct
    println!("{:?}", luca);

    // initialize a point
    let point: Point = Point { x: 10.3, y: 5.0 };

    // access fields
    println!("Coords ({}, {})", point.x, point.y);

    // create new point with update syntax to use field of our first point
    let bottom_right = Point { x: 5.2, ..point };

    println!("Second point ({}, {})", bottom_right.x, bottom_right.y);

    // shadowing for our area example
    let bottom_right = Point { x: 15.2, y: 2.0 };

    // destructure the point and bind to variables
    let Point {
        x: left_edge,
        y: top_edge,
    } = point;

    let rectangle = Rectangle {
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

    println!("The area of {:?} is {}", &rectangle, rect_area(&rectangle));

    println!(
        "The generated square with origin {:?} and dimension {}",
        point, 3.0
    );

    println!("Is {:?}", square(point, 3.0));

    // destructuring example with match
    let point = ThreePoint { xy: (1, 2), z: 3 };

    match point {
        ThreePoint { xy: (1, y), z } => println!("Coords x: 1, y = {} and z = {}", y, z),
        // destructure structs and rename variables (the order doesn't matter)
        ThreePoint { z: 3, xy: i } => println!("z is 3 and x, y = {:?}", i),

        // we can always ignore what we don't need
        ThreePoint { z, .. } => println!("z = {}, we don't need x and y", z),
        // but we need to include the elements we can't do Point { z }
    }
}
