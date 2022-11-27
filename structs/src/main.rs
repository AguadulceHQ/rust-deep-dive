struct TeamMember {
    email: String,
    karma: u64,
    nickname: String,
    on_vacation: bool,
    role: String,
}

fn main() {
    let luca = TeamMember {
        email: String::from("luca@aguadulcehq.com"),
        karma: 42,
        nickname: String::from("Duca"),
        on_vacation: false,
        role: String::from("Founder"),
    };

    println!("Welcome to defining your own types with a struct. How cool is this? 😎");
    println!(
        "Here is my data for example. My nickname is {}, I am {} of Aguadulce and you can find me at {}",
        luca.nickname, luca.role, luca.email
    );
    println!(
        "My karma level is {} and in case you wonder if I am on vacation that's...{} 🤨",
        luca.karma, luca.on_vacation
    );
}
