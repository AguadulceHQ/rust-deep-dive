pub struct ThreadPool;

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
        ThreadPool
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
