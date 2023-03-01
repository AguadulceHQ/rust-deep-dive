//  use declaration can be used to bind a full path to a new name for easier access
// we can also use `as` to bind imports to a different name

use deeply::nested::function as other_function;

fn function() {
    println!("Called `function()`");
}

mod deeply {
    pub mod nested {
        pub fn function() {
            println!("Called `deeply::nested::function()`");
        }
    }
}

fn main() {
    // easier access to deeply::nested::function
    other_function();

    println!("Entering a block");
    {
        use crate::deeply::nested::function;

        // because of use function() defined above the module has been shadowed
        function();

        println!("Leaving the block");
    }
    function();
}
