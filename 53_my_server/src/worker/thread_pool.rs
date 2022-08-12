use super::{Job, Message, Worker};
use crate::pool_error::PoolError;

use std::sync::mpsc;
use std::sync::{Arc, Mutex};

pub struct ThreadPool {
    workers: Vec<Worker>,
    sending_channel: mpsc::Sender<Message>,
}
impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The max_number_of_workers is the number of threads in the pool.
    ///
    /// # Errors
    ///
    /// PoolCreationError : The `new` function will return an error if the max_number_of_workers is zero.
    pub fn new(max_number_of_workers: usize) -> Result<ThreadPool, PoolError> {
        if max_number_of_workers > 0 {
            let mut workers: Vec<Worker> = Vec::with_capacity(max_number_of_workers);

            let (sending_channel, receiving_channel): (
                mpsc::Sender<Message>,
                mpsc::Receiver<Message>,
            ) = mpsc::channel();

            let shared_receiver = Arc::new(Mutex::new(receiving_channel));
            for id in 0..max_number_of_workers {
                workers.push(Worker::new(id, Arc::clone(&shared_receiver)));
            }

            Ok(ThreadPool {
                workers,
                sending_channel,
            })
        } else {
            Err(PoolError::PoolCreationError(
                "The max number of workers should not be 0!".to_string(),
            ))
        }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sending_channel.send(Message::NewJob(job)).unwrap();
        println!("Job sent")
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Sending terminate to workers");
        for _ in &mut self.workers {
            self.sending_channel.send(Message::Terminate).unwrap();
        }
        for worker in &mut self.workers {
            if let Some(handle) = worker.handle.take() {
                handle.join().unwrap();
            }
        }
    }
}
