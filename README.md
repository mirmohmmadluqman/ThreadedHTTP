# ThreadedHTTP

[![Rust](https://img.shields.io/badge/Rust-Programming-orange?style=for-the-badge&logo=rust)](https://www.rust-lang.org/)
[![Build Status](https://img.shields.io/badge/build-passing-brightgreen?style=for-the-badge)](#)
[![License](https://img.shields.io/badge/license-MIT-blue?style=for-the-badge)](#)

A multithreaded HTTP server built from scratch in Rust, demonstrating concurrency patterns, TCP socket handling, and graceful shutdown mechanisms. This project is based on Chapter 21 of *The Rust Programming Language* book.

## Overview

ThreadedHTTP is a basic HTTP server that uses a custom thread pool to handle multiple concurrent requests efficiently. It listens on a TCP socket, parses incoming HTTP requests, and responds with appropriate content based on the requested path.

## Features

- **Multithreaded Request Handling**: Custom thread pool implementation for concurrent request processing
- **TCP Socket Management**: Direct TCP connection handling using Rust's standard library
- **HTTP Request Parsing**: Basic HTTP/1.1 request parsing and response generation
- **Graceful Shutdown**: Clean shutdown mechanism with Ctrl+C signal handling
- **Configurable**: Command-line arguments for port, thread count, and logging
- **Error Handling**: Comprehensive error handling using Result types
- **Fallback Content**: Serves inline HTML when files are not found

## Architecture

```
┌─────────────────┐
│  TCP Listener   │
│  (Port 7878)    │
└────────┬────────┘
         │
         ├──► Incoming Connection
         │
         ▼
┌─────────────────┐
│   Thread Pool   │
│   (4 Workers)   │
└────────┬────────┘
         │
         ├──► Worker Thread 0
         ├──► Worker Thread 1
         ├──► Worker Thread 2
         └──► Worker Thread 3
                │
                ▼
         ┌──────────────┐
         │Request Parser│
         └──────┬───────┘
                │
                ▼
         ┌──────────────┐      ┌─────────┐
         │Path Matching │─────►│GET /    │─► 200 OK
         └──────────────┘      ├─────────┤
                               │GET /sleep│─► 5s delay + 200 OK
                               ├─────────┤
                               │Other    │─► 404 NOT FOUND
                               └─────────┘
                                    │
                                    ▼
                             ┌─────────────┐
                             │HTTP Response│
                             └─────────────┘
                                    │
                                    ▼
                             ┌─────────────┐
                             │   Client    │
                             └─────────────┘
```

## Installation

### Prerequisites

- Rust 1.70 or higher
- Cargo (comes with Rust)

### Building from Source

```bash
git clone https://github.com/mirmohmmadluqman/ThreadedHTTP.git
cd ThreadedHTTP
cargo build --release
```

## Usage

### Basic Usage

Start the server on default port (7878):

```bash
cargo run
```

### Custom Port

```bash
cargo run -- --port 8080
```

### Verbose Mode

Enable detailed logging of incoming requests:

```bash
cargo run -- --verbose
```

### Custom Thread Count

Specify the number of worker threads:

```bash
cargo run -- --threads 8
```

### All Options Combined

```bash
cargo run -- --host 0.0.0.0 --port 8080 --threads 8 --verbose
```

### Command-Line Arguments

```
Options:
  -p, --port <PORT>        Port to listen on [default: 7878]
      --host <HOST>        Host address to bind to [default: 127.0.0.1]
  -t, --threads <THREADS>  Number of worker threads [default: 4]
  -v, --verbose            Enable verbose output
  -h, --help               Print help
  -V, --version            Print version
```

## Testing

Once the server is running, you can test it using:

**Browser**: Navigate to `http://127.0.0.1:7878`

**curl**:
```bash
curl http://127.0.0.1:7878
curl http://127.0.0.1:7878/sleep  # 5-second delayed response
curl http://127.0.0.1:7878/notfound  # 404 response
```

## Learning Objectives

This project demonstrates several key Rust concepts:

1. **Concurrency**: Custom thread pool implementation using channels
2. **TCP Networking**: Direct socket programming with `std::net`
3. **Channels**: Message passing between threads via `mpsc`
4. **Ownership & Borrowing**: Safe memory management across threads
5. **Error Handling**: Proper use of `Result` and `?` operator
6. **Graceful Shutdown**: Atomic boolean for clean thread termination
7. **CLI Development**: Argument parsing with `clap`

## Project Structure

```
ThreadedHTTP/
├── Cargo.toml           # Project dependencies and metadata
├── README.md            # This file
├── src/
│   ├── main.rs         # CLI entry point and server startup
│   └── lib.rs          # Core server logic and thread pool
└── tests/              # Integration tests (optional)
```

## Implementation Details

### Thread Pool

The thread pool maintains a fixed number of worker threads that receive jobs through an MPSC channel. This prevents thread explosion under heavy load while maintaining good concurrency.

### Request Handling

Each worker thread:
1. Waits for a connection from the channel
2. Reads and parses the HTTP request
3. Matches the request path
4. Generates an appropriate response
5. Sends the response back to the client

### Graceful Shutdown

The server uses an `AtomicBool` flag that's set when Ctrl+C is pressed. The main loop checks this flag and cleanly shuts down all worker threads before exiting.

## Performance Characteristics

- **Concurrency**: Handles up to N concurrent requests (where N = thread count)
- **Request Queue**: Unlimited queue depth (bounded by memory)
- **Latency**: Minimal overhead from thread pool dispatch
- **Shutdown Time**: Completes in-flight requests before terminating

## Limitations

This is an educational project with several intentional limitations:

- Only supports HTTP/1.1 GET requests
- No HTTPS/TLS support
- Basic request parsing (not fully HTTP-compliant)
- No keep-alive connections
- Static response content only
- No request body handling

## Future Enhancements

Potential improvements for learning:

- [ ] Add POST request support
- [ ] Implement request body parsing
- [ ] Add static file serving
- [ ] Support for query parameters
- [ ] Request routing with pattern matching
- [ ] Connection pooling and keep-alive
- [ ] Middleware support
- [ ] Logging framework integration
- [ ] Metrics and monitoring
- [ ] TLS/HTTPS support

## Contributing

This is a learning project, but contributions are welcome! Feel free to:

- Report bugs
- Suggest improvements
- Submit pull requests
- Share your own variations

## License

MIT License

Copyright (c) 2025 Mir Mohmmad Luqman

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.

## Acknowledgments

- Inspired by Chapter 21 of [The Rust Programming Language](https://doc.rust-lang.org/book/ch20-00-final-project-a-web-server.html)
- Built as a learning exercise in Rust concurrency and networking

## Resources

- [The Rust Book - Chapter 21](https://doc.rust-lang.org/book/ch20-00-final-project-a-web-server.html)
- [Rust std::net Documentation](https://doc.rust-lang.org/std/net/)
- [Rust std::thread Documentation](https://doc.rust-lang.org/std/thread/)
- [MPSC Channel Documentation](https://doc.rust-lang.org/std/sync/mpsc/)

---

**Made with  Rust**