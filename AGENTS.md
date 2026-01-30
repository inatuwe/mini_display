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

```
mini_display/
├── AGENTS.md                    # This file - project instructions
├── .agents/
│   ├── research/                # Research and reference material
│   │   ├── display_fs_v1_research.md
│   │   └── python_implementation_completed.md
│   └── plans/                   # Implementation plans (TODO/IN-PROGRESS/COMPLETED)
│       └── 001_rust_migration.md
├── src/                         # Python source (legacy, reference only)
│   ├── __init__.py
│   └── com_ports.py
├── tests/                       # Python tests (legacy, reference only)
│   ├── __init__.py
│   └── test_com_ports.py
├── detect_display.py            # Python detection script (legacy)
├── hello_world.py               # Python hello world script (legacy)
└── requirements.txt             # Python dependencies (legacy)
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

- `001_rust_migration.md` - Migrate from Python to Rust (TODO)

## Current Focus: Rust Migration

### Goals

1. Standalone executable - works without runtime dependencies
2. No configuration required - auto-detect display
3. CLI with options for custom text display

### Rust Commands

```bash
# Build
cargo build

# Run
cargo run

# Test
cargo test

# Build release
cargo build --release
```

## Legacy Python Commands

```bash
# Install dependencies
pip install -r requirements.txt

# Run tests
python -m pytest tests/ -v

# Detect display
python detect_display.py

# Show Hello World
python hello_world.py
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
