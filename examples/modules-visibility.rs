// by default items have private visibility
// this can be overriden with pub modifier
// only public items can be accessed outside the module's scope

mod my_custom {
    // items by default have private visibility
    fn private_function() {
        println!("Called `my_custom::private_function()`");
    }

    // with pub we can override the default
    pub fn function() {
        println!("Called `my_custom::function()`");
    }

    // items can access other items in the same module even if private
    pub fn indirect_call() {
        print!("Called `my_custom::indirect_call()`, that\n> ");
        private_function();
    }

    pub fn call_public_function_in_my_mod() {
        print!("called `my_custom::call_public_function_in_my_mod()`, that\n> ");
        nested::public_function_in_my_mod();
        print!("> ");
        nested::public_function_in_super_mod();
    }

    // pub(crate) makes functions visible only within the current crate
    pub(crate) fn public_function_in_crate() {
        println!("called `my_custom::public_function_in_crate()`");
    }

    pub mod nested {
        pub fn function() {
            println!("Called `my_custom::nested::function()`");
        }

        #[allow(dead_code)]
        fn private_function() {
            println!("Called `my_custom::nested::private_function()`");
        }

        // functions decleard using pub(in path) syntax are only visible within the given path.
        // the path must be a parent or ancestor module
        pub(in crate::my_custom) fn public_function_in_my_mod() {
            print!("Called `my_custom::nested::public_function_in_my_mod()`, that\n> ");
            public_function_in_nested();
        }

        // fn declared using pub(self) are only visible within the current module which is equivalent to leaving them private
        pub(self) fn public_function_in_nested() {
            println!("Called `my_custom::nested::public_function_in_nested()`");
        }

        // fn declared using pub(super) are visible within the parent module
        pub(super) fn public_function_in_super_mod() {
            println!("Called `my_custom::nested::public_function_in_super_mod()`");
        }
    }

    mod private_nested {
        #[allow(dead_code)]
        pub fn function() {
            println!("Called `my_custom::private_nested::function()`");
        }

        // private parent items will still restrict the visibility of a child item even if it is declared as visible within a bigger scope
        #[allow(dead_code)]
        pub(crate) fn restricted_function() {
            println!("Called `my_custom::private_nested::restricted_function()`");
        }
    }
}

fn function() {
    println!("Called `function()` not part of modules");
}

fn main() {
    // modules allow to disambiguate between items with same name
    function();
    my_custom::function();

    // public items can be accessed
    my_custom::indirect_call();

    // we can access this because the nested module is public and this item is public
    my_custom::nested::function();

    // pub(crate) items can be called from anywhere in the same crate
    my_custom::public_function_in_crate();
}
