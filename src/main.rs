// use tcp listener
use std::net::TcpListener;
// use tcp stream
use std::net::TcpStream;
// use io
use std::io::prelude::*;
// use thread
use std::thread;

fn main() {
    // create a listener
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    // start accepting connections
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        println!("Connection established!");

        // spawn a new thread to handle the connection
        thread::spawn(|| {
            handle_connection(stream);
        });
    }
}

// read and handle the incoming request
fn handle_connection(mut stream: TcpStream) {
    // buffer to read the request into
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

    // check if the request is a GET request for the root directory
    let get = b"GET / HTTP/1.1\r\n";
    let (status_line, filename) = if buffer.starts_with(get) {
        // if it is, set the status line to 200 OK and the filename to index.html
        ("HTTP/1.1 200 OK\r\n", "index.html")
    } else {
        // otherwise, set the status line to 404 NOT FOUND and the filename to 404.html
        ("HTTP/1.1 404 NOT FOUND\r\n", "404.html")
    };
    
    // read the contents of the file into a string
    let contents = std::fs::read_to_string(filename).unwrap();

    // create the response by combining the status line, content type, and contents of the file
    let response = format!(
        "{}Content-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );

    // write the response to the stream and flush it to ensure that it gets sent immediately
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
 
    println!("Response: {}", response);
}