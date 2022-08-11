use std::borrow::BorrowMut;
use std::error::Error;
use std::fmt::Display;
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Debug)]
pub struct PoolCreationError(String);

impl Display for PoolCreationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Job is a custom type to simplify program reading
type Job = Box<dyn FnOnce() + Send + 'static>;

struct Worker {
    id: usize,
    handle: thread::JoinHandle<()>,
}
impl Worker {
    fn new(id: usize, receiving_channel: Arc<Mutex<mpsc::Receiver<Job>>>) -> Self {
        let handle = thread::spawn(move || loop {
            let job = receiving_channel.lock().unwrap().recv().unwrap();
            println!("Worker {} got a job; executing.", id);
            job();
        });
        Worker { id, handle }
    }
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    sending_channel: mpsc::Sender<Job>,
}

impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The max_number_of_workers is the number of threads in the pool.
    ///
    /// # Errors
    ///
    /// PoolCreationError : The `new` function will return an error if the max_number_of_workers is zero.
    pub fn new(max_number_of_workers: usize) -> Result<ThreadPool, PoolCreationError> {
        if max_number_of_workers > 0 {
            let mut workers: Vec<Worker> = Vec::with_capacity(max_number_of_workers);

            let (sending_channel, receiving_channel): (mpsc::Sender<Job>, mpsc::Receiver<Job>) =
                mpsc::channel();
            let shared_receiver = Arc::new(Mutex::new(receiving_channel));
            for id in 0..max_number_of_workers {
                workers.push(Worker::new(id, Arc::clone(&shared_receiver)));
            }

            Ok(ThreadPool {
                workers,
                sending_channel,
            })
        } else {
            Err(PoolCreationError(
                "The max number of workers should not be 0!".to_string(),
            ))
        }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sending_channel.send(job).unwrap();
    }
}
