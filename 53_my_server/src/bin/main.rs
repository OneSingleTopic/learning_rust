use my_server::ThreadPool;

use std::fs;
use std::io;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::process;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

enum HostMessage {
    Exit,
}
fn main() {
    let mut listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    listener.set_nonblocking(true);

    let (sending_channel, receiving_channel): (
        mpsc::Sender<HostMessage>,
        mpsc::Receiver<HostMessage>,
    ) = mpsc::channel();

    let handle = thread::spawn(move || match ThreadPool::new(4) {
        Ok(pool) => loop {
            if let Ok(_) = receiving_channel.try_recv() {
                println!("STOP Listening");
                break;
            }

            if let Some(Ok(stream)) = listener.incoming().next() {
                println!("New request");
                pool.execute(Box::new(|| handle_connection(stream)));
            };
        },

        Err(err) => {
            eprintln!("Problem creating the pool: {err}");
            process::exit(1);
        }
    });

    loop {
        println!("Shutdown ?");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        match input.trim() {
            "" => {
                sending_channel.send(HostMessage::Exit).unwrap();
                handle.join().unwrap();
                process::exit(0);
            }
            _ => continue,
        };
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";
    let (status, finename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK/r/n/r/n", "index.html")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(10));
        ("HTTP/1.1 200 OK/r/n/r/n", "index.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND/r/n/r/n", "404.html")
    };

    let content = fs::read_to_string(finename).unwrap();
    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status,
        content.len(),
        content
    );
    stream.write(response.as_bytes()).unwrap();
    stream.flush();
}
