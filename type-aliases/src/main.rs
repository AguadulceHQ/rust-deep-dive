// type aliases are useful for long type names
type Thunk = Box<dyn Fn() + Send + 'static>;

fn main() {
    // NPS is now synonm with u8
    // it's not a separate/new type it has same API
    type NPS = u8;
    let score: u8 = 5;
    let nps_score: NPS = 5;

    // they are the same type so we can do usual operations
    println!("The final score is {}", score + nps_score);

    // thunk means code to be evaluated at a later time
    // proper type name for a closure that gets sorted
    let mut f: Thunk = Box::new(|| println!("Hello"));

    does_nothing_with_long_type(f);

    f = does_nothing_and_returns_long_type();
}

fn does_nothing_with_long_type(_f: Thunk) {}

fn does_nothing_and_returns_long_type() -> Thunk {
    let f: Thunk = Box::new((|| println!("World")));
    return f;
}
