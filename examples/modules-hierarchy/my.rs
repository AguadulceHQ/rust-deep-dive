// mod inaccessible and mod nested will located those files and inser them under their respective modules
mod inaccessible;
pub mod nested;

pub fn function() {
    println!("Called `my::function()`");
}

fn private_function() {
    println!("Called `my::private_function()`");
}

pub fn indirect_access() {
    print!("Called `my::indirect_access()`, that\n ");

    private_function();
}
