// this declaration will look for a file named my.rs
// it will insert its contents inside a module named my under this scope

mod my;

fn function() {
    println!("Called `function()`");
}

fn main() {
    my::function();

    function();

    my::indirect_access();

    my::nested::function();
}
