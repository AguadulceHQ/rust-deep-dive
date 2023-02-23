// like if let useful to avoid forcing match in cases where we can do better

fn main() {
    let mut optional = Some(0);

    loop {
        match optional {
            // if optional destructures evaluate the block
            Some(i) => {
                if i > 9 {
                    println!("Great than 9, quit");
                    optional = None;
                } else {
                    println!("i is {:?} try again", i);
                    optional = Some(i + 1);
                } // we are at three levels of indentation here
            }
            // quit the loop when destructuring fails
            _ => {
                break;
            } // do we really need this catch all?
        }
    }

    // we can rewrite the above witha while let loop

    let mut optional_val = Some(0);

    // while let destructures optional into Some(i) evaluate the block otherwise break
    while let Some(i) = optional_val {
        if i > 9 {
            println!("Great than 9, quit");
            optional_val = None;
        } else {
            println!("i is {:?} try again", i);
            optional_val = Some(i + 1);
        } // less indentation and no need if catch all
    }
}
