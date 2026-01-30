# Plan: Display FS V1 (0.96 inch) Python Interaction

## Project Overview
Write a Python program to interact with the Display FS V1 (0.96 inch), detect if it's connected, and display "Hello World!"

---

## Research Summary

### What is Display FS V1 (0.96 inch)?
The **Display FS V1** is a small USB smart display, likely manufactured by **WeAct Studio**. It's a 0.96-inch IPS LCD display that connects via USB-C and communicates through a serial port (COM port on Windows).

**Key Specifications:**
- Screen Size: 0.96 inch
- Connection: USB-C (appears as a serial/COM port)
- Resolution: Typically 160x80 or 128x64 pixels (depending on variant)
- Communication Protocol: USB Serial (UART)

### Supported Libraries/Projects

#### Option 1: turing-smart-screen-python (Recommended)
- **GitHub:** https://github.com/mathoudebine/turing-smart-screen-python
- **Status:** WeAct Studio Display FS 0.96" is **officially supported** (added in recent versions)
- **Features:** 
  - Auto-detect COM port
  - Display images, text, progress bars
  - Screen rotation, brightness control
  - Cross-platform (Windows, Linux, macOS)

#### Option 2: Direct Serial Communication
If the display uses a standard protocol, you can communicate directly using:
- `pyserial` - for serial port communication
- Custom protocol implementation

#### Option 3: SSD1306/SSD1315 (if I2C/SPI OLED)
If this is actually an I2C/SPI OLED display (not USB):
- `adafruit-circuitpython-ssd1306`
- `luma.oled`
- `Pillow` for image creation

---

## Implementation Plan

### Phase 1: Environment Setup
1. **Install Python** (3.9+ recommended)
2. **Install required packages:**
   ```bash
   pip install pyserial
   pip install pillow
   ```

### Phase 2: Device Detection
1. **List available COM ports** to find the display
2. **Check device identification** via serial communication
3. **Verify connection** by sending a test command

### Phase 3: Display "Hello World!"
1. **Initialize the display**
2. **Create an image** with "Hello World!" text
3. **Send the image** to the display

---

## Code Structure

```
mini_display/
├── PLAN_display_fs_v1.md      # This plan file
├── requirements.txt            # Python dependencies
├── detect_display.py           # Script to detect connected display
├── hello_world.py              # Main script to display "Hello World!"
└── display_lib/                # Optional: Custom display library
    ├── __init__.py
    └── display_fs.py
```

---

## Step-by-Step Implementation

### Step 1: Create requirements.txt
```
pyserial>=3.5
pillow>=9.0.0
```

### Step 2: Create detect_display.py
This script will:
- Scan all available COM ports
- Try to identify the Display FS V1
- Report connection status

### Step 3: Create hello_world.py
This script will:
- Connect to the display
- Create an image with "Hello World!" text
- Send the image to the display

---

## Detection Methods

### Method A: COM Port Enumeration (Windows)
```python
import serial.tools.list_ports

def find_display():
    ports = serial.tools.list_ports.comports()
    for port in ports:
        print(f"Port: {port.device}")
        print(f"  Description: {port.description}")
        print(f"  Hardware ID: {port.hwid}")
        print(f"  VID:PID: {port.vid}:{port.pid}")
    return ports
```

### Method B: Using turing-smart-screen-python
```python
# Clone the repository and use their library
# Supports auto-detection of WeAct Studio Display FS
```

---

## Potential Issues & Solutions

| Issue | Solution |
|-------|----------|
| Display not detected | Check USB cable, try different ports, install drivers |
| Permission denied | Run as Administrator (Windows) |
| Wrong COM port | Use device manager to identify correct port |
| Display shows nothing | Check baud rate, verify protocol |
| Driver issues | Install CH340/CH341 drivers (common for USB-serial) |

---

## Alternative Approaches

### If Display FS V1 is an I2C OLED (SSD1306/SSD1315):
- Connect via I2C interface (requires hardware adapter on PC)
- Use Raspberry Pi or Arduino as intermediate controller
- Install `adafruit-circuitpython-ssd1306`

### If using Raspberry Pi:
```bash
sudo apt-get install python3-pip python3-pil
pip3 install adafruit-circuitpython-ssd1306
```

---

## Testing Checklist

- [ ] Python 3.9+ installed
- [ ] Required packages installed
- [ ] Display physically connected via USB-C
- [ ] COM port detected in Device Manager
- [ ] Detection script finds the display
- [ ] "Hello World!" displayed successfully

---

## Resources

1. **turing-smart-screen-python Wiki:** 
   https://github.com/mathoudebine/turing-smart-screen-python/wiki

2. **PySerial Documentation:** 
   https://pyserial.readthedocs.io/

3. **Pillow Documentation:** 
   https://pillow.readthedocs.io/

4. **Adafruit SSD1306 Library:** 
   https://learn.adafruit.com/monochrome-oled-breakouts/python-setup

5. **WeAct Studio Products:**
   https://github.com/WeActStudio

---

## Next Steps

1. ✅ Research completed
2. ⬜ Connect the display to computer
3. ⬜ Run detection script to find COM port
4. ⬜ Implement hello_world.py
5. ⬜ Test and verify display output

---

## Notes

- The exact protocol depends on the display's firmware
- If using turing-smart-screen-python, select "WeActStudio0.96" as the model
- Baud rate is typically 115200 for these displays
- Some displays require initialization before accepting commands

**Last Updated:** January 30, 2026
