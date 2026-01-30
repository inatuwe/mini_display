# Project: Display FS V1 (0.96 inch)

## Overview

Standalone CLI application to interact with the Display FS V1 (0.96 inch), detect if it's connected, and display content.

## Hardware

- **Device:** WeAct Studio Display FS V1 (0.96 inch IPS LCD)
- **Connection:** USB-C (appears as serial/COM port)
- **Resolution:** 160x80 pixels
- **Communication:** USB Serial (UART) at 115200 baud
- **USB Chip:** CH340/CH341 USB-Serial converter
- **Known VID/PID:** CH340 (1A86:7523), CH341 (1A86:5523)

## Project Structure

```text
mini_display/
├── AGENTS.md                    # This file - project instructions
├── Cargo.toml                   # Rust project configuration
├── .agents/
│   ├── research/                # Research and reference material
│   │   ├── display_fs_v1_research.md
│   │   └── python_implementation_completed.md
│   └── plans/                   # Implementation plans (TODO/IN-PROGRESS/COMPLETED)
│       └── 001_rust_migration.md
├── rust-src/                    # Rust source modules
│   ├── main.rs                  # CLI entry point
│   ├── lib.rs                   # Library exports
│   ├── port.rs                  # COM port detection and connection
│   ├── image.rs                 # Image creation and RGB565 conversion
│   └── protocol.rs              # Display command protocol
├── assets/
│   └── fonts/                   # Font files for text rendering
│       └── DejaVuSans.ttf       # Embedded in Rust binary
├── src/                         # Python source modules (reference)
│   ├── __init__.py
│   ├── com_ports.py
│   ├── image.py
│   └── serial_comm.py
├── tests/                       # Python unit tests
│   ├── __init__.py
│   ├── test_com_ports.py
│   └── test_image.py
├── detect_display.py            # Python: Detect connected display
├── display.py                   # Python: Main CLI
└── requirements.txt             # Python dependencies
```

## Plan Management

Plans in `.agents/plans/` follow this workflow:

| Status | Description |
|--------|-------------|
| **TODO** | Planned but not started |
| **IN-PROGRESS** | Currently being worked on |
| **COMPLETED** | Finished and verified |

Each plan file has a `Status:` field at the top to track progress.

### Current Plans

- `001_rust_migration.md` - Migrate from Python to Rust (COMPLETED)

## Rust Commands

```bash
# Install Rust (if not installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Build debug version
cargo build

# Build release version
cargo build --release

# Run directly
cargo run -- "Hello World!"
cargo run -- --detect

# Run release binary
./target/release/display-fs "Hello World!"
./target/release/display-fs --detect
./target/release/display-fs --help
```

## Python Commands

```bash
# Install dependencies
pip install -r requirements.txt

# Run tests
python -m pytest tests/ -v

# Detect display
python detect_display.py

# Show Hello World
python display.py
```

## Git Workflow

Use plain git commands for version control.

```bash
git status
git add -A
git commit -m "Description of changes"
git log --oneline
git push
```

### Commit Guidelines

- Write clear, descriptive commit messages
- Reference plan numbers in commits (e.g., "Plan 001: Initialize Rust project")
- **Commit after each logical step** - Don't wait until everything is done; commit when a phase or meaningful unit of work is complete

## Maintenance

After making changes to the codebase, always:

1. **Update AGENTS.md** - Keep project structure and commands current
2. **Update README.md** - Reflect user-facing changes (new features, usage examples)
3. **Update plan status** - Mark plans as COMPLETED when finished
4. **Run tests** - Verify changes with `python -m pytest tests/ -v`
