# Project: Display FS V1 (0.96 inch) Python Interaction

## Overview
Python program to interact with the Display FS V1 (0.96 inch), detect if it's connected, and display content.

## Hardware
- **Device:** WeAct Studio Display FS V1 (0.96 inch IPS LCD)
- **Connection:** USB-C (appears as serial/COM port)
- **Resolution:** 160x80 pixels
- **Communication:** USB Serial (UART) at 115200 baud
- **USB Chip:** CH340/CH341 USB-Serial converter

## Project Structure
```
mini_display/
├── agents.md              # This file - project information
├── PLAN_display_fs_v1.md  # Research and implementation plan
├── TASKS_display_fs_v1.md # Task breakdown with TDD approach
├── requirements.txt       # Python dependencies
├── src/
│   ├── __init__.py
│   └── com_ports.py       # COM port and serial connection functions
└── tests/
    ├── __init__.py
    └── test_com_ports.py  # Unit tests
```

## Dependencies
- `pyserial>=3.5` - Serial port communication
- `pillow>=9.0.0` - Image creation for display

## Implemented Features

### COM Port Enumeration (Task 2) ✅
- `list_com_ports()` - List all available COM ports
- `format_port_info(port)` - Format port info as readable string

### Display Detection (Task 3) ✅
- `is_display_fs_connected(ports=None)` - Check if Display FS V1 is connected by VID/PID
- `find_display_port(ports=None)` - Find and return the display port object
- Known VID/PID: CH340 (1A86:7523), CH341 (1A86:5523)

### Serial Connection (Task 4) ✅
- `open_connection(port, baud_rate=115200, timeout=1.0)` - Open serial connection
- `close_connection(connection)` - Close serial connection safely

### Image Creation (Task 5) ✅
- `create_blank_image(width, height, bg_color)` - Create blank RGB image
- `draw_text(image, text, position, font_size, color)` - Draw text on image
- `create_hello_world_image()` - Create "Hello World!" image for display
- Display dimensions: 160x80 pixels

### Image to Bytes Conversion (Task 6) ✅
- `image_to_bytes(image)` - Convert PIL image to RGB565 bytes for serial transmission

## Pending Features
- Task 7: Send Data to Display
- Task 6: Image Sending
- Task 7: Hello World Display

## Running Tests
```bash
python -m pytest tests/ -v
```

## Usage Example
```python
from src.com_ports import (
    find_display_port,
    open_connection,
    close_connection,
    is_display_fs_connected
)

# Check if display is connected
if is_display_fs_connected():
    port = find_display_port()
    print(f"Display found on {port.device}")
    
    # Open connection
    conn = open_connection(port)
    
    # ... send data ...
    
    # Close connection
    close_connection(conn)
```

## Git Workflow
Use plain git commands for version control. Do not use GitKraken or other GUI tools.

### Common Commands
```bash
# Check status
git status

# Stage all changes
git add -A

# Commit with message
git commit -m "Description of changes"

# View commit history
git log --oneline

# Push to remote
git push
```

### Commit Guidelines
- Write clear, descriptive commit messages
- Commit after completing each task
- Reference task numbers in commit messages (e.g., "Task 4: Implement serial connection")

