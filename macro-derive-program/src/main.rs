use hello_derive_macro::MacroDerive;
use macro_derive::MacroDerive;

// we use our own custom derive macro
#[derive(MacroDerive)]
struct Developer;

fn main() {
    Developer::hello_derive_macro();
}
