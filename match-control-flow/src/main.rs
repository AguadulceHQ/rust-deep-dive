enum CryptoCoin {
    Bitcoin,
    Altcoin,
    Shitcoin,
}

fn main() {
    let my_crypto = CryptoCoin::Bitcoin;

    println!("Match allows to control the flow according to the value of the variable. I have BTC and it's rating is {} 💰", rating(my_crypto));
}

fn rating(coin: CryptoCoin) -> u8 {
    match coin {
        CryptoCoin::Bitcoin => 100,
        CryptoCoin::Altcoin => 10,
        CryptoCoin::Shitcoin => 0,
    }
}
