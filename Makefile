.PHONY: all build run test clean release help fmt check clippy doc doc-open install run-verbose quick ci

# Default target
all: build

# Build the project
build:
	@echo "Building ThreadedHTTP..."
	@cargo build

# Run the server (default config)
run:
	@cargo run

# Run with verbose logging
run-verbose:
	@cargo run -- --verbose

# Run with custom port
run-port:
	@cargo run -- --port 8080

# Run tests
test:
	@cargo test

# Clean build artifacts
clean:
	@cargo clean

# Build optimized release version
release:
	@cargo build --release
	@echo "Release binary: target/release/threaded_http"

# Format code
fmt:
	@cargo fmt

# Check code without building
check:
	@cargo check

# Run clippy linter
clippy:
	@cargo clippy -- -D warnings

# Generate documentation
doc:
	@cargo doc --no-deps

# Generate and open documentation
doc-open:
	@cargo doc --no-deps --open

# Quick development check (format + check + test)
quick: fmt check test

# Full CI pipeline
ci: fmt clippy test build

# Install to system
install:
	@cargo install --path .

# Show help
help:
	@echo "ThreadedHTTP Makefile Commands:"
	@echo ""
	@echo "  make build        - Build the project"
	@echo "  make run          - Run server on default port (7878)"
	@echo "  make run-verbose  - Run with verbose logging"
	@echo "  make run-port     - Run on port 8080"
	@echo "  make test         - Run tests"
	@echo "  make clean        - Clean build artifacts"
	@echo "  make release      - Build optimized release"
	@echo "  make fmt          - Format code"
	@echo "  make check        - Check code without building"
	@echo "  make clippy       - Run linter"
	@echo "  make doc          - Generate documentation"
	@echo "  make doc-open     - Generate and open docs"
	@echo "  make quick        - Quick dev check (fmt + check + test)"
	@echo "  make ci           - Full CI pipeline"
	@echo "  make install      - Install to system"