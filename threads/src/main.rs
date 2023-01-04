use std::thread;
use std::time::Duration;

fn main() {
    // this method creates a new thread

    thread::spawn(|| {
        for i in 1..10 {
            println!("Web Server Request #{} from the spawned thread", i);
            // allows a different thread to run in the meantime
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("Web Server Main Thread Execution #{}", i);
        thread::sleep(Duration::from_millis(1));
    }
}
