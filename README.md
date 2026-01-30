# Mini Display

Python library to interact with the WeAct Studio Display FS V1 (0.96 inch IPS LCD).

## Features

- Auto-detect display via USB (CH340/CH341 USB-Serial)
- Display text and images on the 160x80 pixel screen
- Cross-platform support (Windows, Linux, macOS)

## Hardware

| Specification | Value |
|---------------|-------|
| Device | WeAct Studio Display FS V1 |
| Screen Size | 0.96 inch IPS LCD |
| Resolution | 160x80 pixels |
| Connection | USB-C (serial) |
| Baud Rate | 115200 |
| USB Chip | CH340/CH341 |

## Installation

```bash
pip install -r requirements.txt
```

### Requirements

- Python 3.9+
- `pyserial>=3.5`
- `pillow>=9.0.0`

### Driver

Install CH340/CH341 USB-Serial drivers if not automatically detected:

- Windows: Usually auto-installed
- macOS: [CH340 Driver](https://github.com/adrianmihalko/ch340g-ch34g-ch34x-mac-os-x-driver)
- Linux: Usually built into the kernel

## Usage

### Detect Display

```bash
python detect_display.py
```

### Display Hello World

```bash
python display.py
```

### Programmatic Usage

```python
from src.com_ports import (
    find_display_port,
    open_connection,
    close_connection,
    is_display_fs_connected
)
from src.image import create_hello_world_image, image_to_bytes
from src.serial_comm import send_image_to_display

# Check if display is connected
if is_display_fs_connected():
    port = find_display_port()
    conn = open_connection(port)
    
    # Create and send image
    image = create_hello_world_image()
    image_data = image_to_bytes(image)
    send_image_to_display(conn, image_data)
    
    close_connection(conn)
```

## Project Structure

```
mini_display/
├── detect_display.py      # Detect connected display
├── display.py             # Main CLI - display text/images
├── requirements.txt       # Python dependencies
├── assets/
│   └── fonts/             # Font files for text rendering
├── src/
│   ├── com_ports.py       # COM port detection and connection
│   ├── image.py           # Image creation and conversion
│   └── serial_comm.py     # Serial communication protocol
└── tests/                 # Unit tests
```

## Running Tests

```bash
python -m pytest tests/ -v
```

## License

MIT License - see [LICENSE](LICENSE) for details.
