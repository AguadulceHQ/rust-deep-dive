pub mod balance_sheet {
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

    mod recoincilation {
        fn pending_transactions() {}

        fn pending_invoices() {}

        fn assign_invoice_to_transaction() {}
    }
}
