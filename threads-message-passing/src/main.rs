// multiple producer single consumer
// multiple sending ends that produce value but only one receiving that consumes

use std::sync::mpsc;
use std::thread;

fn main() {
    // returns a tuple with transmitter and receiver
    // pattern to deconstruct the tuple with let

    let (tx, rx) = mpsc::channel();

    // we move tx into the closure so that the spawned thread owns tx
    thread::spawn(move || {
        let val = String::from("New Project");
        // method that sends a value through the transmitter
        // returns Result<T, E> if the receiver has been dropped it returns an error
        tx.send(val).unwrap();
    });
}
