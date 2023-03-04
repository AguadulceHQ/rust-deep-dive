// to link a crate to a library we can use `extern` flag
// all of its items will be imported under a module named the same as the library
// this is how to link the library through rustc
// rustc crate-use-library.rs --extern crate_library=libcrate_library.rlib

fn main() {
    crate_library::public_function();

    crate_library::indirect_access();
}
