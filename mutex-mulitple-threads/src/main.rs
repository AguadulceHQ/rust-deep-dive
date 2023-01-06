use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    // we wrap the Mutex in the Arc primitive
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        // we can now pass the reference to the mutex and move the ownership
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }
    // counter didn't go out of scope because we have at least one Arc active pointing to it
    // arcs references are stored in handles in the main thread

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Counter reached: {}", *counter.lock().unwrap());
}
