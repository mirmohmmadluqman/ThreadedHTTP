use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    
    println!("Server running on http://127.0.0.1:7878");
    
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    
    let get = b"GET / HTTP/1.1\r\n";
    
    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };
    
    // TODO: Handle file not found error properly
    let contents = fs::read_to_string(filename).unwrap_or_else(|_| {
        String::from("<html><body><h1>Hello from Rust!</h1></body></html>")
    });
    
    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );
    
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}