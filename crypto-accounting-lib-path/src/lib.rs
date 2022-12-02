pub mod balance_sheet {
    pub struct Invoice {
        asset: String,
        amount: f64,
        date: String,
        description: String,
    }

    fn balance_sheet_summary() {}
    pub mod transaction {
        #[derive(Debug)]
        pub struct Transaction {
            pub asset: String,
            pub amount: f64,
            pub invoice: u64,
            pub description: String,
            pub recoinciled: bool,
        }

        fn add_to_expenses() {}

        fn add_to_sales() {}
    }

    pub mod recoincilation {

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
    }
}

pub fn demo_transaction() {
    // relative path to a pub struct
    let demo_transaction = balance_sheet::transaction::Transaction {
        asset: String::from("BTC"),
        amount: 42.0,
        invoice: 1,
        recoinciled: false,
        description: String::from("Payment to Aguadulce to make it shine! 🤑"),
    };
}

pub fn check_invoices() {
    // absolute path
    crate::balance_sheet::recoincilation::pending_invoices();

    // relative path
    balance_sheet::recoincilation::pending_invoices();
}
