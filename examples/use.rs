// use declaration is used to avoid manual scoping

enum Status {
    Rich,
    Poor,
}

enum Work {
    Civilian,
    Soldier,
}

fn main() {
    // use each name so they are available without manual scoping
    use crate::Status::{Poor, Rich};
    // use any name inside Work
    use crate::Work::*;

    // equals to Status::Poor
    let status = Poor;
    let work = Civilian;

    // no scoping needed manually because we brought them into scope
    match status {
        Rich => println!("The rich have lots of money"),
        Poor => println!("The poor have no money but they'll come back!"),
    }
}
