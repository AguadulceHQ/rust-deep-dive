mod balance_sheet {
    struct Invoice {
        asset: String,
        amount: f64,
        date: String,
        description: String,
    }

    fn balance_sheet_summary() {}
    mod transaction {
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
            // no need of super to use a method in scope
            pending_transactions();
        }
    }
}

pub fn check_invoices() {
    // absolute path
    crate::balance_sheet::recoincilation::pending_invoices();

    // relative path
    balance_sheet::recoincilation::pending_invoices();
}
