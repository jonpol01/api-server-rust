// use tcp listener
use std::net::TcpListener;
// use tcp stream
use std::net::TcpStream;
// use io
use std::io::prelude::*;
// use thread
use std::thread;
// use time
//use std::time::Duration;



fn main() {
    // create a listener
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        println!("Connection established!");

        thread::spawn(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

    let get = b"GET / HTTP/1.1\r\n";

    // validate then open index.html
    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n", "index.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n", "404.html")
    };
    
    // read file
    let contents = std::fs::read_to_string(filename).unwrap();

    // create valid http/1.1 response
    let response = format!(
        "{}Content-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );

    // write response
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
 
    println!("Response: {}", response);
}
