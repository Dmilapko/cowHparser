# Makefile for cow-parser
.PHONY: all build run test clean check pre-commit

# Default command
all: build

# Build the project
build:
    cargo build

# Run the main application
run: build
    ./target/debug/cow-parser example.cow

# Run unit tests
test:
    cargo test

# Clean the project
clean:
    cargo clean

# Format the code
fmt:
    cargo fmt

# Check the code for errors
check:
    cargo check

# Run clippy for linting
clippy:
    cargo clippy -- -D warnings

# A pre-commit hook to ensure quality
pre-commit:
    make fmt
    make test