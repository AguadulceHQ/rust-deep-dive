trait Backend {
    fn code(&self);
    // associated non-method fn
    fn intro() -> String;
}

trait Frontend {
    fn code(&self);
    // associated non-method fn
    fn intro() -> String;
}

struct Developer;

impl Frontend for Developer {
    fn code(&self) {
        println!("Coding a pixel perfect interface folks 🧑‍🎨")
    }

    // associated non-method fn
    fn intro() -> String {
        String::from("I am a frontend developer")
    }
}

impl Backend for Developer {
    fn code(&self) {
        println!("Coding the most scalable API ever 🤖")
    }

    // associated non-method fn
    fn intro() -> String {
        String::from("I am a backend developer")
    }
}

impl Developer {
    fn code(&self) {
        println!("Mmm...just give me ☕")
    }
    // associated non-method fn
    fn intro() -> String {
        String::from("I am a versatile developer")
    }
}

fn main() {
    // the compiler defaults to the method defined on the type directly equivalent to Developer::code(&team_member)
    let team_member = Developer;
    team_member.code();
    // if we want to call the methods of other traits we need to be more specific
    Backend::code(&team_member);
    Frontend::code(&team_member);

    // we can call the associated fn defined on Developer
    println!(
        "An introduction for a generic dev is: {}",
        Developer::intro()
    );

    // to call instead a specific trait implemented on Developer we need the fully qualified syntax
    // we want to call the intro method from the Backend trait as implemented on Dog
    // we want to treat the Dog type as an Backend for this fn call
    // in general the fqs is <Type as Trait>::function(receiver_if_method, next_arg, ...);
    println!(
        "A proper introduction for a backend dev is: {}",
        <Developer as Backend>::intro()
    );
}
