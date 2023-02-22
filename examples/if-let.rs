// for some use cases match is not the best

fn main() {
    let optional = Some(7);

    match optional {
        // we need two levels of indentation to get i
        Some(i) => {
            println!("We got {:?}", i);
        }
        // we need to handle all cases, waste space
        _ => {}
    }

    // we can instead use the if let construct
    let number = Some(7);
    let emoji: Option<i32> = None;

    if let Some(i) = number {
        println!("Matched {:?}", i);
    }

    // to provide another condition to test
    let another = false;

    if let Some(i) = emoji {
        println!("Matched {:?}", i);
    } else if another {
        println!("Didn't match a number but the other condition");
    } else {
        println!("Didn't match a number and not even the other condition");
    }
}
