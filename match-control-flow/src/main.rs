enum CryptoCoin {
    Bitcoin,
    Altcoin,
    Shitcoin,
}

fn main() {
    let my_crypto = CryptoCoin::Bitcoin;
    let my_altcoin = CryptoCoin::Altcoin;

    println!("Match allows to control the flow according to the value of the variable. I have BTC and it's rating is {} 💰", rating(my_crypto));
    println!("We can also use a catch-all pattern for those values that we don't want to handle explicitly. I have ETH and it's rating is {} 🙀", maximalist_rating(my_altcoin));
}

fn rating(coin: CryptoCoin) -> u8 {
    match coin {
        CryptoCoin::Bitcoin => 100,
        CryptoCoin::Altcoin => 10,
        CryptoCoin::Shitcoin => 0,
    }
}

fn maximalist_rating(coin: CryptoCoin) -> u8 {
    match coin {
        CryptoCoin::Bitcoin => 100,
        _ => 0,
    }
}
