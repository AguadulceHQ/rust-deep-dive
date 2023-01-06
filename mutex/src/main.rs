use std::sync::Mutex;

fn main() {
    // create a new Mutex<T>
    let favorite = Mutex::new(42);

    {
        // acquire the lock
        // this blocks the current thread so that it can't do any work until it gets the lock
        // this fails if another thread with the lock panicked this is why we use unwrap
        let mut number = favorite.lock().unwrap();

        // we have the lock we can change the value, without the lock we won't be able to do this
        *number = 3;
    }
    // thanks to Drop trait of MutexGuard the lock has been released automatically once number is out of scope
    println!("The favorite number has been changed to {:?}", favorite);
}
