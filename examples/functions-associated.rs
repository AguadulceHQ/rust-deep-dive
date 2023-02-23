// associated functions are fn defined on a type generally
// methods are associated functions called on a particular instance of a type

struct Point {
    x: f64,
    y: f64,
}

impl Point {
    // associated function because it's associated to all instances of ype Point
    fn origin() -> Point {
        Point { x: 0.0, y: 0.0 }
    }

    fn new(x: f64, y: f64) -> Point {
        Point { x: x, y: y }
    }
}

struct Rectangle {
    p1: Point,
    p2: Point,
}

impl Rectangle {
    // method
    // &self is sugar syntax for self: &Self where Self is the type of the caller object in this case Self = Rectangle
    fn area(&self) -> f64 {
        // self gives access to the struct fields via the dot operator
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        // abs is a method that returns absolute value of the caller
        ((x1 - x2) * (y1 - y2)).abs()
    }

    fn translate(&mut self, x: f64, y: f64) {
        self.p1.x += x;
        self.p2.x += x;

        self.p1.y += y;
        self.p2.y += y;
    }
}

// Pair owns resources, two heap allocated integers
struct Pair(Box<i32>, Box<i32>);

impl Pair {
    // this method consumes the resources of the caller object
    // self desugars to self: Self
    fn destroy(self) {
        // destructure self
        let Pair(first, second) = self;

        println!("Destroying Pair({}, {})", first, second);
    }
}

fn main() {
    let rectangle = Rectangle {
        // Associated functions are called using double colons
        p1: Point::origin(),
        p2: Point::new(3.0, 4.0),
    };

    // Methods are called using the dot operator
    // Note that the first argument `&self` is implicitly passed, i.e.
    // `rectangle.area()` === `Rectangle::area(&rectangle)`
    println!("Rectangle area: {}", rectangle.area());

    let mut square = Rectangle {
        p1: Point::origin(),
        p2: Point::new(1.0, 1.0),
    };

    // Okay! Mutable objects can call mutable methods
    square.translate(1.0, 1.0);

    let pair = Pair(Box::new(1), Box::new(2));

    pair.destroy();
}
