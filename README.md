# ThreadedHTTP

[![Rust](https://img.shields.io/badge/Rust-Programming-orange?style=for-the-badge&logo=rust)](https://www.rust-lang.org/)
[![Build Status](https://img.shields.io/badge/build-passing-brightgreen?style=for-the-badge)](#)
[![License](https://img.shields.io/badge/license-MIT-blue?style=for-the-badge)](#)

A multithreaded HTTP server built from scratch in Rust, demonstrating concurrency patterns, TCP socket handling, and graceful shutdown mechanisms. This project is based on Chapter 21 of *The Rust Programming Language* book.

## Overview(Userstory)

ThreadedHTTP is a basic HTTP server that uses a custom thread pool to handle multiple concurrent requests efficiently. It listens on a TCP socket, parses incoming HTTP requests, and responds with appropriate content based on the requested path.

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
## License

MIT License
