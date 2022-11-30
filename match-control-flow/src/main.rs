enum CryptoCoin {
    Bitcoin,
    Altcoin,
    Shitcoin(MarketCapitalization),
}

enum MarketCapitalization {
    High,
    Mid,
    Low,
    Micro,
}

fn main() {
    let my_crypto = CryptoCoin::Bitcoin;
    let my_altcoin = CryptoCoin::Altcoin;
    let my_shitcoin = CryptoCoin::Shitcoin(MarketCapitalization::High);

    println!("Match allows to control the flow according to the value of the variable. I have BTC and it's rating is {} 💰", rating(my_crypto));
    println!("We can also use a catch-all pattern for those values that we don't want to handle explicitly. I have ETH and it's rating is {} 🙀", maximalist_rating(my_altcoin));
    println!("Match is great to use with enum and bind to their values so that we can run some code. For example my shitcoin may not be that bad 👉 {} 🙏", shit_rating(my_shitcoin));
}

fn rating(coin: CryptoCoin) -> u8 {
    match coin {
        CryptoCoin::Bitcoin => 100,
        CryptoCoin::Altcoin => 10,
        CryptoCoin::Shitcoin(_) => 0,
    }
}

fn maximalist_rating(coin: CryptoCoin) -> u8 {
    match coin {
        CryptoCoin::Bitcoin => 100,
        _ => 0,
    }
}

fn shit_rating(coin: CryptoCoin) -> u8 {
    match coin {
        CryptoCoin::Bitcoin => 100,
        CryptoCoin::Altcoin => 10,
        CryptoCoin::Shitcoin(market_cap) => {
            if matches!(market_cap, MarketCapitalization::High) {
                println!("At least sounds like a high market cap 💩");
                return 10;
            }
            0
        }
    }
}
