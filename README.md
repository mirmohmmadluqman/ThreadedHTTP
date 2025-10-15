# ThreadedHTTP

A multithreaded HTTP server built with Rust as a learning project.

## Goal
Build a web server that can handle multiple concurrent requests using thread pools.

## Status
🚧 Work in Progress(Under Devlopment)

## Current Progress
- [x] Basic TCP listener setup
- [x] HTTP request parsing (basic)
- [x] Response generation
- [ ] Thread pool implementation
- [ ] Concurrent request handling

## Running
```bash
cargo run
```

Then visit http://127.0.0.1:7878 in your browser.

## Issues I'm Working On
- Need to implement thread pool (currently single-threaded)
- Figure out how to handle multiple connections simultaneously
- Understanding mpsc channels for worker communication

## Learning Resources
Following Chapter 21 of The Rust Programming Language Book.