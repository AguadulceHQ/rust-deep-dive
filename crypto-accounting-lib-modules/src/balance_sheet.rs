// we are not in crate root the compiler will look for subfolder to find modules' code in
// balance_sheet/module_name.rs
pub mod recoincilation;
pub mod transaction;

pub struct Invoice {
    asset: String,
    amount: f64,
    date: String,
    description: String,
}

fn balance_sheet_summary() {}

pub fn demo_transaction() {
    // relative path to a pub struct
    let demo_transaction = transaction::Transaction {
        asset: String::from("BTC"),
        amount: 42.0,
        invoice: 1,
        recoinciled: false,
        description: String::from("Payment to Aguadulce to make it shine! 🤑"),
    };

    dbg!(demo_transaction);
}
