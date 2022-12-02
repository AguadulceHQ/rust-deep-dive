use crypto_accounting_lib_path::balance_sheet::transaction::Transaction;

fn main() {
    let first_transaction = Transaction {
        asset: String::from("BTC"),
        amount: 42.0,
        invoice: 1,
        recoinciled: false,
        description: String::from("Payment to Aguadulce to make it shine! 🤑"),
    };

    dbg!(first_transaction);
}
