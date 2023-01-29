use std::thread;
pub struct ThreadPool {
    // we need a place to store threads
    // JoinHandle is returned by spawn method
    // we return a the unit type () because the closure we pass to the thread pool handle the connection and don't return anything
    threads: Vec<thread::JoinHandle<()>>,
}

impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// Size is the number of threads in the pool.
    /// Size is unsigned as it doesn't make sense to have negative threads
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> ThreadPool {
        // for simplicity we want this to be an unrecoverable error
        // otherwise we could have used build with a Result as a return type
        // pub fn build(size: usize) -> Result<ThreadPool, PoolCreationError>
        assert!(size > 0);

        // create a fixed sized vector to hold the threads
        let mut threads = Vec::with_capacity(size);

        for _ in 0..size {
            // create threads and store them in the vector
        }

        ThreadPool { threads }
    }

    // we emulate the signature from spawn because we want a similar behaviour
    // we want to accept closures with F and we take them with the FnOnce trait bound
    // we also need Send trait to be implmented on F to be sure we can pass the closure between threads
    // we also need a static lifetime annotation as we don't know how long the thread will take to execute the closure so it needs to be in scope
    pub fn execute<F>(&self, f: F)
    where
        // () after FnOnce because a closure that takes no parameters but returns a unit type
        F: FnOnce() + Send + 'static,
    {
    }
}
