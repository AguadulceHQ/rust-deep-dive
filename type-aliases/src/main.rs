fn main() {
    // NPS is now synonm with u8
    // it's not a separate/new type it has same API
    type NPS = u8;
    let score: u8 = 5;
    let nps_score: NPS = 5;

    // they are the same type so we can do usual operations
    println!("The final score is {}", score + nps_score);
}
