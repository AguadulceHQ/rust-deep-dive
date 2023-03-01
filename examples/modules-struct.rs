// structs have an extra level of visibility with their fields which default to private
// this is to ensure encapsulation (information hiding by default)

mod my_custom {
    // a public struct with a public field of generic type T
    pub struct OpenBox<T> {
        pub contents: T,
    }

    // a public struct with a private field of generic type T
    pub struct ClosedBox<T> {
        contents: T,
    }

    impl<T> ClosedBox<T> {
        // a public constructor
        pub fn new(contents: T) -> ClosedBox<T> {
            ClosedBox { contents: contents }
        }
    }
}

fn main() {
    // public structs with public fields are easy
    let open_box = my_custom::OpenBox {
        contents: "Public info",
    };

    println!("The open box contains: {}", open_box.contents);

    // public structs with private fields cannot be constructed using field names

    let _closed_box = my_custom::ClosedBox::new("Classified");
}
