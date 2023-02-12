use std::{
    sync::{Arc, mpsc, Mutex},
    thread,
};

// Define the `ThreadPool` struct which will store the worker threads and a sender channel
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

// Define a type alias for the job that the worker threads will execute
type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    // Initialize a new `ThreadPool` with the specified number of worker threads
    pub fn new(size: usize) -> ThreadPool {
        // Ensure that the specified size is greater than 0
        assert!(size > 0);

        // Create a sender and receiver channel for communication between the `ThreadPool` and the worker threads
        let (sender, receiver) = mpsc::channel();

        // Wrap the receiver channel in an Arc (Atomic Reference Counted) and Mutex to allow for multiple worker threads to access the receiver channel simultaneously
        let receiver = Arc::new(Mutex::new(receiver));

        // Create a vector to store the worker threads
        let mut workers = Vec::with_capacity(size);

        // Create `size` number of worker threads and store them in the `workers` vector
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        // Return the `ThreadPool` struct
        ThreadPool {
            workers,
            sender: Some(sender),
        }
    }

    // Execute a given closure `f` in a worker thread
    pub fn execute<F>(&self, f: F)
        where
            F: FnOnce() + Send + 'static,
    {
        // Box the closure `f` to make it usable as a trait object
        let job = Box::new(f);

        // Send the boxed closure to a worker thread through the sender channel
        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}

// Define the `Drop` trait implementation for the `ThreadPool` struct to allow for proper cleanup when the `ThreadPool` goes out of scope
impl Drop for ThreadPool {
    fn drop(&mut self) {
        // Drop the sender channel to signal to the worker threads that there will be no more jobs to execute
        drop(self.sender.take());

        // Iterate over the worker threads and shut them down
        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            // If the worker thread has not already been joined, join it to wait for its completion
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

// Define the `Worker` struct to store the worker thread's ID and join handle
struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    // Initialize a new worker thread with the specified ID and receiver channel
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        // Create a new thread that will run a loop
        let thread = thread::spawn(move || loop {
            // Get the next message from the receiver,
            // If the receiver is empty this will block until a message is sent
            let message = receiver.lock().unwrap().recv();

            match message {
                Ok(job) => {
                    println!("Worker {id} got a job; executing.");

                    // Call the closure that was received as a job
                    job();
                }
                Err(_) => {
                    println!("Worker {id} disconnected; shutting down.");
                    // if the receiver is dropped, the channel is disconnected
                    // and this worker will exit
                    break;
                }
            }
        });

        // Create and return a new worker instance
        Worker {
            id,
            thread: Some(thread),
        }
    }
}