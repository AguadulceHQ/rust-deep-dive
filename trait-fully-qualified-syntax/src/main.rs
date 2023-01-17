trait Backend {
    fn code(&self);
}

trait Frontend {
    fn code(&self);
}

struct Developer;

impl Frontend for Developer {
    fn code(&self) {
        println!("Coding a pixel perfect interface folks 🧑‍🎨")
    }
}
impl Backend for Developer {
    fn code(&self) {
        println!("Coding the most scalable API ever 🤖")
    }
}

impl Developer {
    fn code(&self) {
        println!("Mmm...just give me ☕")
    }
}

fn main() {
    // the compiler defaults to the method defined on the type directly equivalent to Developer::code(&team_member)
    let team_member = Developer;
    team_member.code();
    // if we want to call the methods of other traits we need to be more specific
    Backend::code(&team_member);
    Frontend::code(&team_member);
}
