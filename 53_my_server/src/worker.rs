use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;

pub mod thread_pool;

enum Message {
    NewJob(Job),
    Terminate,
}

/// Job is a custom type to simplify program reading
type Job = Box<dyn FnOnce() + Send + 'static>;

struct Worker {
    id: usize,
    handle: Option<thread::JoinHandle<()>>,
}
impl Worker {
    fn new(id: usize, receiving_channel: Arc<Mutex<mpsc::Receiver<Message>>>) -> Self {
        let handle = thread::spawn(move || loop {
            // Because we use let and that there is no other mention to the Mutex in the block,
            // it will be released at the end of the let statement.
            // If, however, we stored the channel is a variable in a first step, then we would
            // have to wait until the end of the block (after the job call) to release the Mutex,
            // hence no multi-threading.
            let message = receiving_channel.lock().unwrap().recv().unwrap();
            match message {
                Message::NewJob(job) => {
                    println!("Worker {} got a job; executing.", id);
                    job();
                }
                Message::Terminate => {
                    println!("Worker {} received a Terminate message - SHUTTING DOWN", id);
                    break;
                }
            }
        });
        Worker {
            id,
            handle: Some(handle),
        }
    }
}
