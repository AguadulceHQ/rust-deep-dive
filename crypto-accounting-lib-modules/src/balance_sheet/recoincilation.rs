
fn pending_transactions() {
    // use super to access parent context
    super::balance_sheet_summary();
}

pub fn pending_invoices() {
    let invoice = super::Invoice {
        asset: String::from("USD"),
        amount: 0.00,
        date: String::from("Today"),
        description: String::from("Default"),
    };

    // no need of super to use a method in scope
    pending_transactions();
}
