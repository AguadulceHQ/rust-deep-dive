// we need this in scope because we are using thread::JoinHandle as the type of items in the vector in ThreadPool
use std::{sync::mpsc, thread};
pub struct ThreadPool {
    // we need a place to store threads
    // we refactor to use Workers because otherwise we had to pass immediately some code to run to the thread
    // workers will store the thread and then we can pass closures to them to be executed in their thread
    workers: Vec<Worker>,
    // sender part of the channel which hold the message queue of jobs
    sender: mpsc::Sender<Job>,
}

struct Job;

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

        // crete a new channel/message queue for this ThreadPool
        let (sender, receiver) = mpsc::channel();

        // create a fixed sized vector to hold the threads
        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            // create workers and store them in the vector
            workers.push(Worker::new(id));
        }

        ThreadPool { workers, sender }
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

// a Worker holds a running thread to which it can pass code later
// id is to distinguish workers one from the other
// this doesn't need to be public the interface for the client will be the ThreadPool not its internal implementation
struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize) -> Worker {
        // here we actually create a thread that keeps running
        let thread = thread::spawn(|| {});

        Worker { id, thread }
    }
}
