fn main() {
    let citizen: (u8, char) = (200, 'L');
    let (citizen_identifier, citizen_initial) = citizen;

    println!(
        "We can access a tuple's value by deconstructing 👉 {} 🏗️",
        citizen_identifier
    );
    println!("And by dot notation too 👉 {} 🤯", citizen_initial);

    let default_array = [100, 200, 400, 500];
    let quarters_profit: [i32; 4] = [100, 200, 400, 500];

    println!(
      "This variable didn't have an explicit type and length assigned and it defaults to the properties of the array assigned to it for example 👉 {} 🦺",
      default_array[0] 
    );

    println!(
        "You can access an array by indexing 👉 {} 🔍",
        quarters_profit[0]
    );
}
