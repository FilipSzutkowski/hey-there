use std::{
    fs,
    io::{prelude::BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
    process, thread,
    time::Duration,
};

use hey_there::ThreadPool;

fn main() {
    let listener = TcpListener::bind("localhost:42069").unwrap();
    let t_pool = match ThreadPool::new(4) {
        Ok(pool) => pool,
        Err(err) => {
            eprintln!("Application Error: {err}");
            process::exit(1);
        }
    };

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        t_pool.execute(|| {
            handle_connection(stream);
        });
    }

    println!("Application shutting down.");
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "index.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "index.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();
    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
