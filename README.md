# Mini Display

CLI tool to interact with the WeAct Studio Display FS V1 (0.96 inch IPS LCD).

## Features

- **Standalone executable** - No runtime dependencies, just download and run
- Auto-detect display via USB (CH340/CH341 USB-Serial)
- Display text on the 160x80 pixel screen
- Cross-platform support (Windows, Linux, macOS)

## Hardware

| Specification | Value                       |
|---------------|------------------------------|
| Device        | WeAct Studio Display FS V1  |
| Screen Size   | 0.96 inch IPS LCD           |
| Resolution    | 80x160 pixels (portrait)    |
| Connection    | USB-C (serial)              |
| Baud Rate     | 115200                      |
| USB Chip      | CH340/CH341                 |

## Quick Start

```bash
# Display text
./display-fs show "Hello World!"

# Auto-fit text to largest readable size
./display-fs show --auto "Hi"

# Check if display is connected
./display-fs show --detect

# Custom font size
./display-fs show -s 20 "Big Text"

# Show current Spotify track (macOS)
./display-fs spotify --loop

# Run a preset (clock, git status, etc.)
./display-fs preset clock --loop

# Demo mode: cycle through all presets
./display-fs demo
```

## Installation

### Option 1: Download Binary (Recommended)

Download the latest release for your platform from [Releases](https://github.com/inatuwe/mini_display/releases).

### Option 2: Build from Source

Requires [Rust](https://rustup.rs/):

```bash
# Install Rust (if not installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Build release binary (Latin only, ~1.5 MB)
cargo build --release

# Build with Japanese/CJK support (~6.5 MB)
cargo build --release --features japanese

# Binary is at: ./target/release/display-fs
```

#### Japanese/CJK Font Support

The default build uses DejaVuSans font (~750 KB) for Latin text. For Japanese song titles or CJK characters, build with the `japanese` feature:

```bash
# Using just (recommended)
just build-jp       # Build with Japanese support
just install-jp     # Install japanese-enabled binary

# Or with cargo
cargo build --release --features japanese
```

| Build | Font | Binary Size |
|-------|------|-------------|
| Default | DejaVuSans | ~1.5 MB |
| `--features japanese` | Noto Sans JP | ~6.5 MB |

### Driver

Install CH340/CH341 USB-Serial drivers if not automatically detected:

- **Windows:** Usually auto-installed
- **macOS:** [CH340 Driver](https://github.com/adrianmihalko/ch340g-ch34g-ch34x-mac-os-x-driver)
- **Linux:** Usually built into the kernel

## Usage

```text
display-fs show [OPTIONS] [TEXT]

Arguments:
  [TEXT]  Text to display [default: "Hello World!"]

Options:
  -s, --font-size <SIZE>        Font size in pixels [default: 14]
  -a, --auto                    Auto-fit text to largest readable size
  -o, --orientation <MODE>      Display orientation: landscape or portrait [default: landscape]
  -d, --delay <SECONDS>         Delay between pages [default: 2.0]
  -l, --loop                    Loop display continuously
      --detect                  Only check if display is connected
  -h, --help                    Print help
```

### Auto-Fit Mode

The `--auto` flag automatically calculates the largest font size that fits your text on the 80x160 pixel display. Great for maximizing readability:

```bash
# Short text displays large
./display-fs show --auto "Hi"        # Uses ~70px font

# Longer text uses smaller font to fit
./display-fs show --auto "Hello!"    # Uses ~40px font
```

### Orientation Mode

The `--orientation` flag switches between landscape (160x80, default) and portrait (80x160) modes:

```bash
# Landscape (default) - wider display
./display-fs show --auto "Wide text"

# Portrait - taller display, good for multi-line
./display-fs show --auto -o portrait "Line 1\nLine 2\nLine 3"
```

### Examples

```bash
# Default message
./display-fs show

# Custom message
./display-fs show "Hello from Rust!"

# Auto-fit (recommended)
./display-fs show --auto "Status OK"

# Portrait orientation with auto-fit
./display-fs show --auto -o portrait "Tall"

# Larger font (manual)
./display-fs show -s 24 "BIG"

# Just detect display
./display-fs show --detect
```

### Spotify Now Playing (macOS)

Display the currently playing Spotify track:

```bash
# Show once
./display-fs spotify

# Live updates (refresh every 2 seconds)
./display-fs spotify --loop

# Faster refresh
./display-fs spotify --loop --speed fast
```

### Presets

Run built-in system information presets:

```bash
# List available presets
./display-fs presets

# Run a preset
./display-fs preset clock
./display-fs preset git
./display-fs preset ip
./display-fs preset whoami

# Loop a preset (live updates)
./display-fs preset cpu --loop

# Demo mode: cycle through all presets
./display-fs demo
```

Available presets: `clock`, `datetime`, `uptime`, `git`, `ip`, `whoami`, `pwd`, `cpu`, `memory`, `docker`, `spotify`, `fortune`

## Project Structure

```text
mini_display/
├── Cargo.toml             # Rust project configuration
├── src/                   # Rust source code
│   ├── main.rs            # CLI entry point
│   ├── lib.rs             # Library exports
│   ├── port.rs            # USB port detection
│   ├── image.rs           # Image creation & RGB565
│   ├── protocol.rs        # Display protocol
│   ├── spotify.rs         # Spotify now-playing (macOS)
│   └── text.rs            # Text wrapping & pagination
└── assets/
    └── fonts/             # Font files (embedded in binary)
```

## License

MIT License - see [LICENSE](LICENSE) for details.
