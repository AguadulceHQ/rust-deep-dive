
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
