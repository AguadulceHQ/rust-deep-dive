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
}
