// the compiler will look for src/balance_sheet.rs because we are in the crate root
pub mod balance_sheet;

pub use crate::balance_sheet::recoincilation::pending_invoices;

pub fn check_invoices() {
    // absolute path
    crate::balance_sheet::recoincilation::pending_invoices();

    // relative path
    balance_sheet::recoincilation::pending_invoices();
}
