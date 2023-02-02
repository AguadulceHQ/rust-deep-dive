// we need this in scope because we are using thread::JoinHandle as the type of items in the vector in ThreadPool
use std::{
    sync::{mpsc, Arc, Mutex},
    thread::{self, Thread},
};
pub struct ThreadPool {
    // we need a place to store threads
    // we refactor to use Workers because otherwise we had to pass immediately some code to run to the thread
    // workers will store the thread and then we can pass closures to them to be executed in their thread
    workers: Vec<Worker>,
    // sender part of the channel which hold the message queue of jobs
    sender: mpsc::Sender<Job>,
}

// we use type alias for this longer type
type Job = Box<dyn FnOnce() + Send + 'static>;

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

        let receiver = Arc::new(Mutex::new(receiver));

        // create a fixed sized vector to hold the threads
        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            // create workers and store them in the vector
            workers.push(Worker::new(id, Arc::clone(&receiver)));
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
        // new Job instance that contains the closure we pass to execute
        let job = Box::new(f);
        // send through the channel the job that needs to get down
        self.sender.send(job).unwrap();
    }
}

// prevent stopping threads abruptly with CTRL+C
// this gets executed when exiting main
impl Drop for ThreadPool {
    // self is a mutable reference and we need to be able to mutate worker so we need &mut
    fn drop(&mut self) {
        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);
            // we'll take the option's value
            if let Some(thread) = worker.thread.take() {
                // join on worker's thread, if the call fails we use unwrap to make Rust panic and go in ungraceful shutdown
                thread.join().unwrap();
            }
        }
    }
}

// a Worker holds a running thread to which it can pass code later
// id is to distinguish workers one from the other
// this doesn't need to be public the interface for the client will be the ThreadPool not its internal implementation
struct Worker {
    id: usize,
    // a Worker that is running will have a Some variant in thread
    // so when we want to clean up the thread we replace Some with None
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    // add receiver to the channel that gets sent to the thread
    // all threads will be receiver ends of the channel
    // on which ThreadPool can send Jobs
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        // here we actually create a thread that keeps running
        let thread = thread::spawn(move || loop {
            // we lock on the receiver to acquire the mutex
            // we unwrap to panic in case of errors (e.g. poisoned state)
            // we call recv to receive a Job from the channel
            // a final unwrap to move past any errors e.g. the sender has shut down
            let job = receiver.lock().unwrap().recv().unwrap();

            println!("Worker {id} got a job, under execution");

            // we finally execute the closure passed
            job();
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}
