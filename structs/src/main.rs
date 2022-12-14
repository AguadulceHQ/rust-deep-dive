#[derive(Debug)]
struct TeamMember {
    email: String,
    karma: u64,
    nickname: String,
    on_vacation: bool,
    role: String,
}

struct Availability(char, char);

fn main() {
    let luca = TeamMember {
        email: String::from("luca@aguadulcehq.com"),
        karma: 42,
        nickname: String::from("Duca"),
        on_vacation: false,
        role: String::from("Founder"),
    };

    println!("Welcome to defining your own types with a struct. How cool is this? ๐");
    println!(
        "Here is my data for example. My nickname is {}, I am {} of Aguadulce and you can find me at {}",
        luca.nickname, luca.role, luca.email
    );
    println!(
        "My karma level is {} and in case you wonder if I am on vacation that's...{} ๐คจ",
        luca.karma, luca.on_vacation
    );

    let luca_clone = TeamMember { karma: 100, ..luca };

    println!(
        "We needed to clone Luca and so we did ๐ค. The new Luca has a karma score of {} ๐คฏ",
        luca_clone.karma
    );

    let luca_clone_availability = Availability('M', 'S');

    println!("Using a tuple struct we understood that Luca's clone is available from {} to {}...he is a bot! ๐ฆพ", luca_clone_availability.0, luca_clone_availability.1);

    let Availability(start, finish) = luca_clone_availability;

    println!("And just a friendly reminder as we are here that we can deconstruct tuple's value and in fact they are the same from {} to {} ๐", start, finish);
    println!("Let's test out the dbg! macro on our struct ๐");

    dbg!(&luca_clone);

    println!("We can also debug single expressions to make sure they match our expectations ๐คฏ");

    dbg!(luca_clone.karma * 2);
}
