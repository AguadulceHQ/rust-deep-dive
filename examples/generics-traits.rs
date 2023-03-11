struct Empty;
struct Null;

trait DoubleDrop<T> {
    // define a method on the caller type which takes an additional single parameter T and does nothing with it
    fn double_drop(self, _: T);
}

// implement DoubleDrop<T> for any generic parameter T and caller U
impl<T, U> DoubleDrop<T> for U {
    // this method takes ownership of both arguments and deallocates both
    fn double_drop(self, _: T) {}
}

fn main() {
    let empty = Empty;
    let null = Null;

    // deallocate empty and null
    empty.double_drop(null);
}
