// using a nested path here to condense the use statement
// we bring in scope Transaction with a full path
use crypto_accounting_lib_path::balance_sheet::{self, transaction::Transaction};
use rand::Rng;

fn main() {
    let first_transaction = Transaction {
        asset: String::from("BTC"),
        amount: rand::thread_rng().gen_range(1.0..=100.0),
        invoice: 1,
        recoinciled: false,
        description: String::from("Payment to Aguadulce to make it shine! 🤑"),
    };

    dbg!(first_transaction);

    balance_sheet::demo_transaction();
}
