use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

mod request;
mod response;

fn main() {
    let port: u16 = 8080;
    let listener = TcpListener::bind(format!("127.0.0.1:{}", port)).unwrap();
    println!("Server started on port 8080");

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        thread::spawn(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let request = String::from_utf8_lossy(&buffer[..]);
    let response = response::build_response(request.to_string());

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();

    if let Some(request_line) = request.lines().next() {
        println!("Handled request: {}", request_line);
    }
}