use std::error::Error;
use std::fmt;
use std::fs;
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

type Job = Box<dyn FnOnce() + Send + 'static>;

enum Message {
    NewJob(Job),
    Terminate,
}

struct Worker {
    #[allow(dead_code)]
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv();

            match message {
                Ok(Message::NewJob(job)) => job(),
                Ok(Message::Terminate) | Err(_) => break,
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Message>>,
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool {
            workers,
            sender: Some(sender),
        }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        if let Some(sender) = &self.sender {
            let _ = sender.send(Message::NewJob(job));
        }
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        if let Some(sender) = &self.sender {
            for _ in &self.workers {
                let _ = sender.send(Message::Terminate);
            }
        }

        drop(self.sender.take());

        for worker in &mut self.workers {
            if let Some(thread) = worker.thread.take() {
                let _ = thread.join();
            }
        }
    }
}

#[derive(Debug)]
pub struct ServerError {
    message: String,
}

impl ServerError {
    pub fn new(msg: &str) -> ServerError {
        ServerError {
            message: msg.to_string(),
        }
    }
}

impl fmt::Display for ServerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Server error: {}", self.message)
    }
}

impl Error for ServerError {}

pub struct ServerConfig {
    pub address: String,
    pub pool_size: usize,
    pub verbose: bool,
}

impl Default for ServerConfig {
    fn default() -> Self {
        ServerConfig {
            address: "127.0.0.1:7878".to_string(),
            pool_size: 4,
            verbose: false,
        }
    }
}

pub fn start_server(
    config: ServerConfig,
    shutdown: Arc<AtomicBool>,
) -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind(&config.address)?;
    listener.set_nonblocking(true)?;

    if config.verbose {
        println!("Server listening on {}", config.address);
    }

    let pool = ThreadPool::new(config.pool_size);

    for stream in listener.incoming() {
        if shutdown.load(Ordering::Relaxed) {
            if config.verbose {
                println!("Shutdown signal received, stopping server...");
            }
            break;
        }

        match stream {
            Ok(stream) => {
                let verbose = config.verbose;
                pool.execute(move || {
                    if let Err(e) = handle_connection(stream, verbose) {
                        eprintln!("Error handling connection: {}", e);
                    }
                });
            }
            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                thread::sleep(Duration::from_millis(10));
                continue;
            }
            Err(e) => {
                eprintln!("Connection error: {}", e);
            }
        }
    }

    if config.verbose {
        println!("Server shutdown complete");
    }

    Ok(())
}

fn handle_connection(mut stream: TcpStream, verbose: bool) -> Result<(), Box<dyn Error>> {
    let buf_reader = BufReader::new(&stream);
    let request_line = buf_reader
        .lines()
        .next()
        .ok_or_else(|| ServerError::new("Empty request"))??;

    if verbose {
        println!("Request: {}", request_line);
    }

    let (status_line, filename) = match request_line.as_str() {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "hello.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };

    let contents = fs::read_to_string(filename).unwrap_or_else(|_| {
        if filename == "hello.html" {
            "<!DOCTYPE html>
            <html>
            <head>
                <title>Hello</title>
            </head>
            <body>
            <h1>Hello, world!</h1>
            <p>Welcome to ThreadedHTTP server</p>
            </body>
            </html>".to_string()
        } else {
            "<!DOCTYPE html>
            <html>
            <head>
                <title>404</title>
            </head>
            <body>
            <h1>404 Not Found</h1>
            <p>The requested resource was not found.</p>
            </body>
            </html>".to_string()
        }
    });

    let length = contents.len();
    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes())?;
    stream.flush()?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_thread_pool_creation() {
        let pool = ThreadPool::new(4);
        assert_eq!(pool.workers.len(), 4);
    }

    #[test]
    #[should_panic]
    fn test_thread_pool_zero_size() {
        ThreadPool::new(0);
    }

    #[test]
    fn test_thread_pool_execute() {
        let pool = ThreadPool::new(2);
        let (tx, rx) = mpsc::channel();

        pool.execute(move || {
            tx.send(42).unwrap();
        });

        let result = rx.recv_timeout(Duration::from_secs(1));
        assert_eq!(result.unwrap(), 42);
    }
}