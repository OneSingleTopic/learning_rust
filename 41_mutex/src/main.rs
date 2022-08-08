use std::borrow::Borrow;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let mut handles = vec![];
    let mutex = Arc::new(Mutex::new(0));
    for i in 0..10 {
        let mutex_clone = Arc::clone(&mutex);
        let handle = thread::spawn(move || {
            let mut x = mutex_clone.lock().unwrap();
            *x += 1;
        });

        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }

    println!("The locked value is now : {}", mutex.lock().unwrap());
}
