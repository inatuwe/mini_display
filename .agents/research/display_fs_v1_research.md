# Research: Display FS V1 (0.96 inch)

## What is Display FS V1 (0.96 inch)?
The **Display FS V1** is a small USB smart display, likely manufactured by **WeAct Studio**. It's a 0.96-inch IPS LCD display that connects via USB-C and communicates through a serial port (COM port on Windows).

**Key Specifications:**
- Screen Size: 0.96 inch
- Connection: USB-C (appears as a serial/COM port)
- Resolution: 160x80 pixels
- Communication Protocol: USB Serial (UART) at 115200 baud
- USB Chip: CH340/CH341 USB-Serial converter
- Known VID/PID: CH340 (1A86:7523), CH341 (1A86:5523)

---

## Supported Libraries/Projects

### Option 1: turing-smart-screen-python (Recommended for Python)
- **GitHub:** https://github.com/mathoudebine/turing-smart-screen-python
- **Status:** WeAct Studio Display FS 0.96" is **officially supported** (added in recent versions)
- **Features:** 
  - Auto-detect COM port
  - Display images, text, progress bars
  - Screen rotation, brightness control
  - Cross-platform (Windows, Linux, macOS)

### Option 2: Direct Serial Communication
If the display uses a standard protocol, you can communicate directly using:
- `pyserial` (Python) - for serial port communication
- `serialport` (Rust) - for serial port communication in Rust
- Custom protocol implementation

### Option 3: SSD1306/SSD1315 (if I2C/SPI OLED)
If this is actually an I2C/SPI OLED display (not USB):
- Python: `adafruit-circuitpython-ssd1306`, `luma.oled`, `Pillow`
- Rust: `ssd1306` crate

---

## Potential Issues & Solutions

| Issue | Solution |
|-------|----------|
| Display not detected | Check USB cable, try different ports, install drivers |
| Permission denied | Run as Administrator (Windows) or add user to dialout group (Linux) |
| Wrong COM port | Use device manager to identify correct port |
| Display shows nothing | Check baud rate, verify protocol |
| Driver issues | Install CH340/CH341 drivers (common for USB-serial) |

---

## Resources

1. **turing-smart-screen-python Wiki:** 
   https://github.com/mathoudebine/turing-smart-screen-python/wiki

2. **PySerial Documentation:** 
   https://pyserial.readthedocs.io/

3. **Rust serialport crate:**
   https://docs.rs/serialport/latest/serialport/

4. **WeAct Studio Products:**
   https://github.com/WeActStudio

---

## Notes

- The exact protocol depends on the display's firmware
- If using turing-smart-screen-python, select "WeActStudio0.96" as the model
- Baud rate is typically 115200 for these displays
- Some displays require initialization before accepting commands

**Last Updated:** January 30, 2026
