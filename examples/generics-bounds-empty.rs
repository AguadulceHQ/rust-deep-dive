// because of how bounds work even if a trait doesn't include any functionality
// you can use it as a bound
// Eq and Copy are example of such traits from the std library

struct Cardinal;
struct BlueJay;
struct Turkey;

trait Red {}
trait Blue {}

// doesn't atter that these traits are empty
impl Red for Cardinal {}
impl Blue for BlueJay {}

// these fn are valid only for types which implements these traits
fn red<T: Red>(_: &T) -> &'static str {
    "red"
}
fn blue<T: Blue>(_: &T) -> &'static str {
    "blue"
}

fn main() {
    let cardinal = Cardinal;
    let blue_jay = BlueJay;
    let _turkey = Turkey;

    println!("A cardinal is {}", red(&cardinal));
    println!("A blue jay is {}", blue(&blue_jay));
}
