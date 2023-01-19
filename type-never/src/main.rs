// fn never never returns
// we cannot create values of type ! so bar can never return
fn never() -> ! {
    panic!();
}

enum Option<T> {
    Some(T),
    None,
}

use crate::Option::*;

impl<T> Option<T> {
    // val is type T
    // panic! doesn't produce a value it ends the program
    // so in the None arm we don't return a value from unwrap hence the code is valid
    pub fn unwrap(self) -> T {
        match self {
            Some(val) => val,
            None => panic!("called `Option::unwrap()` on a `None` value"),
        }
    }
}

fn main() {
    let mut num = 4;

    loop {
        match num {
            4 => {
                num = 42;
                break;
            }
            // the reason why the compiler doesn't complain is because continue returns a never value !
            // ! can be coerced into any other type
            // because it doesn't return a value the control moves back to the top of the loop
            _ => continue,
        };
    }

    println!("Let's see if we got our magic number {}", num);

    let t = Option::Some(String::from("Gotcha!"));
    println!("Let's see what we got...{}", t.unwrap());

    never();
}
