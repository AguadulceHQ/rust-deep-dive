fn main() {
    check_balance();

    accounts_list();
}

// demonstration of if let pattern and else if let
fn check_balance() {
    let bank_account: Option<&str> = None;
    let is_monday = false;
    let age: Result<u8, _> = "18".parse();

    if let Some(account) = bank_account {
        println!("You can use your {account}");
    } else if is_monday {
        println!("You cannot use your account on Monday");
    } else if let Ok(age) = age {
        if age > 18 {
            println!("You can access your account as you are an adult");
        } else {
            println!("You can't use your bank account you are a minor");
        }
    } else {
        println!("Do you have a bank account in the first place?");
    }
}

// demonstaration of while let conditional loops
fn accounts_list() {
    let mut accounts = Vec::new();

    accounts.push(123);
    accounts.push(456);
    accounts.push(789);

    println!("Displaying your accounts");

    while let Some(top) = accounts.pop() {
        println!("Account #{}", top);
    }
}
