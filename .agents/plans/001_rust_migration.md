# Plan 001: Migrate to Rust

**Status:** IN-PROGRESS  
**Priority:** High  
**Created:** January 30, 2026

---

## Objective

Migrate the Display FS V1 application from Python to Rust to create a standalone executable that works without any configuration.

## Goals

1. **Standalone executable** - Works without any runtime dependencies
2. **No configuration required** - Auto-detect display and work out of the box
3. **CLI extension** - Accept inputs like what text to show on the display

---

## Phase 1: Basic Rust Setup (DONE)

- [x] Initialize Rust project with `cargo init --name display-fs`
- [x] Configure Cargo.toml with dependencies
- [x] Create module structure (lib.rs, main.rs, port.rs, image.rs, protocol.rs)
- [x] Embed DejaVuSans.ttf font using `include_bytes!`

## Phase 2: Port Detection - `port.rs` (DONE)

Port from: `src/com_ports.py`

- [x] Define VID/PID constants:
  - CH340: (0x1A86, 0x7523)
  - CH341: (0x1A86, 0x5523)  
  - WeAct: (0x1A86, 0xFE0C)
- [x] `list_ports()` - Enumerate available serial ports
- [x] `find_display_port()` - Find port matching VID/PID
- [x] `is_display_connected()` - Boolean check
- [x] `open_connection()` - Open serial at 115200 baud, 1s timeout
- [x] `close_connection()` - Not needed (Rust drops connection automatically)

## Phase 3: Image Creation - `image.rs` (DONE)

Port from: `src/image.py`

- [x] Constants: WIDTH=160, HEIGHT=80
- [x] `create_blank_image()` - Black 160x80 RGB image
- [x] `draw_text()` - Draw text with embedded font, auto-center
- [x] `create_text_image(text, font_size)` - Convenience function
- [x] `image_to_rgb565_bytes()` - Convert to RGB565 little-endian:
  - R: 5 bits (>> 3), G: 6 bits (>> 2), B: 5 bits (>> 3)
  - Pack: (r5 << 11) | (g6 << 5) | b5
  - Output: low byte first (little-endian)

## Phase 4: Display Protocol - `protocol.rs` (DONE)

Port from: `src/serial_comm.py`

- [x] Constants:
  - CMD_SET_BITMAP = 0x05
  - CMD_END = 0x0A
  - CHUNK_SIZE = 160 * 4 = 640 bytes
- [x] `create_bitmap_header()` - 10-byte header:
  - Byte 0: 0x05
  - Bytes 1-2: x0 (0) little-endian
  - Bytes 3-4: y0 (0) little-endian
  - Bytes 5-6: x1 (159) little-endian
  - Bytes 7-8: y1 (79) little-endian
  - Byte 9: 0x0A
- [x] `send_image_to_display()` - Send header, then data in 640-byte chunks
- [x] Flush buffers before sending, 100ms delay after

## Phase 5: CLI Application - `main.rs` (DONE)

Port from: `display.py`

- [x] Clap derive-based argument parsing
- [x] Positional arg: text to display (default: "Hello World!")
- [x] `--font-size` / `-s`: Font size (default: 14)
- [x] `--detect` / `-d`: Just check if display is connected
- [x] User-friendly error messages (no display found, connection failed)
- [x] Exit code 0 on success, 1 on failure

## Phase 6: Build & Test (DONE)

- [x] Install Rust: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
- [x] `cargo build --release` - Compiles successfully
- [x] Test with actual display on macOS - Working!
- [x] Verify binary size is reasonable - 1.5 MB
- [ ] Update README.md with Rust usage instructions

---

## Rust Dependencies

```toml
[package]
name = "display-fs"
version = "0.1.0"
edition = "2021"

[dependencies]
serialport = "4.7"      # Serial port communication
image = "0.25"          # Image creation
imageproc = "0.25"      # Text drawing on images
ab_glyph = "0.2"        # Font handling (modern replacement for rusttype)
clap = { version = "4", features = ["derive"] }
thiserror = "2"         # Error handling

[profile.release]
strip = true
lto = true
```

---

## File Structure

```text
mini_display/
├── Cargo.toml
├── rust-src/               # Rust sources (separate from Python src/)
│   ├── main.rs             # CLI entry point
│   ├── lib.rs              # Library exports
│   ├── port.rs             # Port enumeration & detection
│   ├── image.rs            # Image creation & RGB565 conversion
│   └── protocol.rs         # Display command protocol
├── assets/
│   └── fonts/
│       └── DejaVuSans.ttf  # Embedded in binary
└── src/                    # Python sources (keep for reference)
```

---

## Implementation Reference

### VID/PID Detection (from Python)

```python
DISPLAY_FS_VID_PID = [
    (0x1A86, 0x7523),  # CH340
    (0x1A86, 0x5523),  # CH341
    (0x1A86, 0xFE0C),  # WeAct Studio Display FS V1
]
```

### RGB565 Conversion (from Python)

```python
r5 = (r >> 3) & 0x1F
g6 = (g >> 2) & 0x3F
b5 = (b >> 3) & 0x1F
rgb565 = (r5 << 11) | (g6 << 5) | b5
# Little-endian output
data.append(rgb565 & 0xFF)
data.append((rgb565 >> 8) & 0xFF)
```

### Protocol Header (from Python)

```python
header[0] = 0x05           # CMD_SET_BITMAP
header[1:3] = x0 (LE)      # 0
header[3:5] = y0 (LE)      # 0
header[5:7] = x1 (LE)      # 159
header[7:9] = y1 (LE)      # 79
header[9] = 0x0A           # CMD_END
```

---

## Notes

- Keep Python implementation for reference during migration
- Use `rust-src/` directory to avoid conflicts with Python `src/`
- Embed font in binary for zero-config deployment
- Focus on macOS first, then cross-platform
