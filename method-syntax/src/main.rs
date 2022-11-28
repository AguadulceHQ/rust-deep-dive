#[derive(Debug)]
struct TeamMember {
    email: String,
    karma: u64,
    nickname: String,
    on_vacation: bool,
    role: String,
}

impl TeamMember {
    fn create(name: String) -> Self {
        Self {
            email: String::from("Default"),
            karma: 0,
            on_vacation: false,
            nickname: name,
            role: String::from("Default"),
        }
    }

    fn create_monster(karma: u64) -> Self {
        Self {
            email: String::from("Default"),
            karma,
            on_vacation: false,
            nickname: String::from("Monster"),
            role: String::from("Default"),
        }
    }

    fn on_vacation(&self) -> String {
        let vacation_message = format!(
            "{} vacation status is {} reach out at {}",
            self.nickname, self.on_vacation, self.email
        );
        vacation_message
    }

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

    let luca_clone = TeamMember::create(String::from("Luca The Clone"));

    println!(
        "We cloned Luca through an associated function which is not a method. Can we know his profession? {} 🤯",
        luca_clone.profession()
    );

    println!(
        "Let's try also if Rust is able to handle an associated fn with same name as a field {}",
        luca_clone.on_vacation()
    );

    let monster = TeamMember::create_monster(666);

    println!("We created a monster and his karma is {} 🤯", monster.karma);
}
