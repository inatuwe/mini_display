# Rust Migration Requirements

## Overview

Migrate the Display FS V1 CLI application from Python to Rust.

## Requirements

### 0. Migrate from Python to Rust

- Rewrite the existing Python implementation in Rust
- Maintain same functionality: detect display, show content
- Use Rust's serial port libraries for USB communication

### 1. Standalone Executable (Zero Configuration)

- Single binary with no external dependencies
- Auto-detect the display without user configuration
- Scan USB serial ports for CH340/CH341 chips (VID/PID: 1A86:7523, 1A86:5523)
- Works out of the box on macOS, Linux, and Windows

### 2. CLI with Text Input

- Accept command-line arguments for custom text
- Example: `display-fs "Hello World"`
- Options for text positioning, clearing screen, etc.

## Technical Considerations

- **Serial Library:** `serialport` crate for cross-platform USB serial
- **CLI Parsing:** `clap` crate for argument parsing
- **Cross-compilation:** Support for multiple platforms via `cross` or native builds

## Reference

- Existing Python implementation in `src/com_ports.py`, `detect_display.py`, `hello_world.py`
- Hardware research in `.agents/research/display_fs_v1_research.md`
