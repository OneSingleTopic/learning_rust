use my_server::ThreadPool;
use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::process;
use std::thread;
use std::time::Duration;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    match ThreadPool::new(4) {
        Ok(pool) => {
            for stream in listener.incoming() {
                let mut stream = stream.unwrap();

                pool.execute(Box::new(|| handle_connection(stream)));
            }
        }
        Err(err) => {
            eprintln!("Problem creating the pool: {err}");
            process::exit(1);
        }
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
        thread::sleep(Duration::from_secs(3));
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
