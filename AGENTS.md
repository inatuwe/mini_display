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
│   ├── plans/                   # Implementation plans
│   │   ├── todo/                # Planned but not started
│   │   ├── in-progress/         # Currently being worked on
│   │   └── completed/           # Finished and verified
│   └── skills/                  # Agent skills
│       ├── ralph/               # Autonomous implementation loops
│       └── research/            # Deep research workflow
├── src/                         # Rust source modules
│   ├── main.rs                  # CLI entry point
│   ├── lib.rs                   # Library exports
│   ├── port.rs                  # COM port detection and connection
│   ├── image.rs                 # Image creation and RGB565 conversion
│   └── protocol.rs              # Display command protocol
└── assets/
    └── fonts/                   # Font files for text rendering
        └── DejaVuSans.ttf       # Embedded in Rust binary
```

## Plan Management

Plans in `.agents/plans/` follow this workflow:

| Status | Description |
|--------|-------------|
| **TODO** | Planned but not started |
| **IN-PROGRESS** | Currently being worked on |
| **COMPLETED** | Finished and verified |

Each plan file has a `Status:` field at the top to track progress.

## Commands

### Quick Commands (using `just`)

```bash
# Install just (if not installed)
brew install just  # or: cargo install just

# Show available commands
just

# Development workflow
just check         # Fast type-check (no codegen)
just lint          # Run clippy lints
just fmt           # Format code
just test          # Run tests
just ci            # Full check: fmt + lint + test

# Build and run
just build         # Build release binary
just install       # Build and update ./display-fs
just run "Hi"      # Run with custom text
just detect        # Detect display
```

### Direct Cargo Commands

```bash
# Install Rust (if not installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Build
cargo build              # Debug build
cargo build --release    # Release build

# Run
cargo run -- "Hello World!"
cargo run -- --detect

# Quality checks
cargo fmt
cargo clippy -- -D warnings
cargo test
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
4. **Run tests** - Verify changes with `cargo test`
