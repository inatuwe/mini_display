# List available commands
default:
    @just --list

# Check code compiles (fast)
check:
    cargo check

# Run all lints
lint:
    cargo clippy -- -D warnings

# Format code
fmt:
    cargo fmt

# Run tests
test:
    cargo test

# Full check: fmt, lint, test
ci: fmt lint test

# Build release binary
build:
    cargo build --release

# Run with text
run text="Hello World!":
    cargo run -- "{{text}}"

# Detect display
detect:
    cargo run -- --detect
