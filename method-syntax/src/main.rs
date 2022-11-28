#[derive(Debug)]
struct TeamMember {
    email: String,
    karma: u64,
    nickname: String,
    on_vacation: bool,
    role: String,
}

impl TeamMember {
    fn profession(&self) -> String {
        let profession = format!("{} is a {}", self.nickname, self.role);
        profession
    }

    fn reputation(&self, other_member: &TeamMember) -> bool {
        self.karma > other_member.karma
    }
}

fn main() {
    let luca = TeamMember {
        email: String::from("luca@aguadulcehq.com"),
        karma: 42,
        nickname: String::from("Duca"),
        on_vacation: false,
        role: String::from("Founder"),
    };

    let luca_not_as_good = TeamMember {
        email: String::from("luca@aguadulcehq.com"),
        karma: 1,
        nickname: String::from("Duca"),
        on_vacation: false,
        role: String::from("Founder"),
    };

    println!("Welcome to defining behaviour for your own types. How cool is this? 😎");

    println!(
        "Through syntax method we discovered that {} 😎",
        luca.profession()
    );

    println!(
        "Let's compare our Lucas. Is the 1st better than the 2nd? {} 🏅",
        luca.reputation(&luca_not_as_good)
    );
}
