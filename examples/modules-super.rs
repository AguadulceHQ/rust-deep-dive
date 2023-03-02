// super and self kw can be used in the path to remove ambiguity when accessing items and prevent unnecessary hardcoding of paths

fn function() {
    println!("Called `function()`");
}

mod cool {
    pub fn function() {
        println!("Called `cool::function()`");
    }
}

mod my {
    fn function() {
        println!("Called `my::function()`");
    }

    mod cool {
        // need to be pub to be accessible also from the parent scope
        pub fn function() {
            println!("Called `my::cool::function()`");
        }
    }

    pub fn indirect_call() {
        println!("Called my::indirect_call()`, that\n>");

        // self refers to the current module in scope
        // this will call function() on this module
        self::function(); // equivalent to function();
        function();

        // we can also use self and access another module inside the scope
        self::cool::function();

        // super kw refers to the parent scope (outside the my module)
        super::function();

        // bind cool::function to the crate scope
        // crate scope is the outermost scope
        {
            use crate::cool::function as root_function;
            root_function();
        }
    }
}

fn main() {
    my::indirect_call();
}
