use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

use rust_httpserver::ThreadPool;

fn main() {
    // Create a TcpListener bound to the IP address 127.0.0.1 and port 5000
    let listener = TcpListener::bind("127.0.0.1:5000").unwrap();

    // Create a thread pool with 4 worker threads
    let pool = ThreadPool::new(4);

    // Listen for incoming connections and handle each connection in a separate thread
    // The `take(2)` method is used to limit the number of connections to 2 for demonstration purposes
    for stream in listener.incoming().take(2) {
        // Get the stream from the incoming connection
        let stream = stream.unwrap();

        // Send a closure to the thread pool for execution
        // The closure will handle the connection
        pool.execute(|| {
            handle_connection(stream);
        });
    }

    // The main thread will wait for all worker threads to complete and then shut down
    println!("Shutting down.");
}

fn handle_connection(mut stream: TcpStream) {
    // Create a buffer with a size of 1024 bytes
    let mut buffer = [0; 1024];

    // Read data from the stream into the buffer
    stream.read(&mut buffer).unwrap();

    // Check if the request starts with "GET / HTTP/1.1\r\n"
    let get = b"GET / HTTP/1.1\r\n";
    let (status_line, filename) = if buffer.starts_with(get) {
        // If it does, set the status line to "HTTP/1.1 200 OK" and the filename to "pages/index.html"
        ("HTTP/1.1 200 OK", "pages/index.html")
    } else {
        // If not, set the status line to "HTTP/1.1 404 NOT FOUND" and the filename to "pages/404.html"
        ("HTTP/1.1 404 NOT FOUND", "pages/404.html")
    };

    // Read the contents of the file into a string
    let contents = fs::read_to_string(filename).unwrap();

    // Create the response string
    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );

    // Write the response to the stream
    stream.write_all(response.as_bytes()).unwrap();

    // Flush the stream
    stream.flush().unwrap();
}
