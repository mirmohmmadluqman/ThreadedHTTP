use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    
    println!("Server running on http://127.0.0.1:7878");
    
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        
        println!("Connection established!");
    }
}