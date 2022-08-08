use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();

    thread::spawn(move || {
        let ducks: Vec<String> = vec![
            String::from("Duck 0 !"),
            String::from("Duck 1 !"),
            String::from("Duck 2 !"),
            String::from("Duck 3 !"),
        ];
        for duck in ducks {
            tx.send(duck).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
        //Impossible as ducks is moved by for loop and cannot be borrowed because of sender
        // println!("ducks : {}", ducks[3]);
    });

    thread::spawn(move || {
        let ducks: Vec<String> = vec![
            String::from("Girafe 0 !"),
            String::from("Girafe 1 !"),
            String::from("Girafe 2 !"),
            String::from("Girafe 3 !"),
        ];
        for duck in ducks {
            tx1.send(duck).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
        //Impossible as ducks is moved by for loop and cannot be borrowed because of sender
        // println!("ducks : {}", ducks[3]);
    });

    let value = rx.recv().unwrap();
    println!("Got : {}", value);
    for received in rx {
        println!("Got : {}", received);
    }
}

fn move_ownership() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();
}
fn say_hi() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from spawned thread ! ", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    handle.join().unwrap();

    for i in 1..5 {
        println!("hi number {} from main thread ! ", i);
        thread::sleep(Duration::from_millis(1));
    }
}
