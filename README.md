# Mini Display

CLI tool to interact with the WeAct Studio Display FS V1 (0.96 inch IPS LCD).

## Features

- **Standalone executable** - No runtime dependencies, just download and run
- Auto-detect display via USB (CH340/CH341 USB-Serial)
- Display text on the 160x80 pixel screen
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

## Quick Start

```bash
# Display text
./display-fs "Hello World!"

# Check if display is connected
./display-fs --detect

# Custom font size
./display-fs -s 20 "Big Text"
```

## Installation

### Option 1: Download Binary (Recommended)

Download the latest release for your platform from [Releases](https://github.com/inatuwe/mini_display/releases).

### Option 2: Build from Source

Requires [Rust](https://rustup.rs/):

```bash
# Install Rust (if not installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Build release binary
cargo build --release

# Binary is at: ./target/release/display-fs
```

### Driver

Install CH340/CH341 USB-Serial drivers if not automatically detected:

- **Windows:** Usually auto-installed
- **macOS:** [CH340 Driver](https://github.com/adrianmihalko/ch340g-ch34g-ch34x-mac-os-x-driver)
- **Linux:** Usually built into the kernel

## Usage

```
display-fs [OPTIONS] [TEXT]

Arguments:
  [TEXT]  Text to display [default: "Hello World!"]

Options:
  -s, --font-size <SIZE>  Font size in pixels [default: 14]
  -d, --detect            Only check if display is connected
  -h, --help              Print help
```

### Examples

```bash
# Default message
./display-fs

# Custom message
./display-fs "Hello from Rust!"

# Larger font
./display-fs -s 24 "BIG"

# Just detect display
./display-fs --detect
```

## Project Structure

```
mini_display/
├── Cargo.toml             # Rust project configuration
├── rust-src/              # Rust source code
│   ├── main.rs            # CLI entry point
│   ├── lib.rs             # Library exports
│   ├── port.rs            # USB port detection
│   ├── image.rs           # Image creation & RGB565
│   └── protocol.rs        # Display protocol
├── assets/
│   └── fonts/             # Font files (embedded in binary)
└── src/                   # Python reference implementation
```

## License

MIT License - see [LICENSE](LICENSE) for details.
