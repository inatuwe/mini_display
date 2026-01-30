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

# Build and install local binary
install: build
    rm -f ./display-fs
    cp target/release/display-fs ./display-fs
    @echo "✓ Installed ./display-fs"

# Run with text
run text="Hello World!":
    cargo run -- "{{text}}"

# Detect display
detect:
    cargo run -- --detect
