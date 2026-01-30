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
├── .agents/
│   ├── research/                # Research and reference material
│   │   ├── display_fs_v1_research.md
│   │   └── python_implementation_completed.md
│   └── plans/                   # Implementation plans (TODO/IN-PROGRESS/COMPLETED)
│       └── 001_rust_migration.md
├── assets/
│   └── fonts/                   # Font files for text rendering
│       └── DejaVuSans.ttf
├── src/                         # Python source modules
│   ├── __init__.py
│   ├── com_ports.py             # COM port detection and connection
│   ├── image.py                 # Image creation and conversion
│   └── serial_comm.py           # Serial communication protocol
├── tests/                       # Unit tests
│   ├── __init__.py
│   ├── test_com_ports.py
│   └── test_image.py
├── detect_display.py            # Detect connected display
├── display.py                   # Main CLI - display text/images
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

- `001_rust_migration.md` - Migrate from Python to Rust (TODO)

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

## Maintenance

After making changes to the codebase, always:

1. **Update AGENTS.md** - Keep project structure and commands current
2. **Update README.md** - Reflect user-facing changes (new features, usage examples)
3. **Update plan status** - Mark plans as COMPLETED when finished
4. **Run tests** - Verify changes with `python -m pytest tests/ -v`
