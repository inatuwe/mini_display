# Plan 001: Migrate to Rust

**Status:** TODO  
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

## Phase 1: Basic Rust Setup (TODO)

- [ ] Initialize Rust project with Cargo
- [ ] Add dependencies: `serialport`, `image`, `clap`
- [ ] Create basic project structure
- [ ] Set up testing framework

## Phase 2: Core Functionality (TODO)

- [ ] Implement COM port enumeration
- [ ] Implement display detection by VID/PID
- [ ] Implement serial connection open/close
- [ ] Implement image creation (blank image, text drawing)
- [ ] Implement RGB565 conversion
- [ ] Implement display command protocol
- [ ] Implement send bytes to display

## Phase 3: CLI Application (TODO)

- [ ] Basic CLI with `clap` 
- [ ] `--detect` flag to check if display is connected
- [ ] `--text "message"` to display custom text
- [ ] `--help` for usage information
- [ ] Error handling with user-friendly messages

## Phase 4: Build & Distribution (TODO)

- [ ] Build release binary
- [ ] Test on macOS
- [ ] Cross-compile for Windows/Linux (optional)
- [ ] Create README for Rust version

---

## Rust Dependencies

```toml
[dependencies]
serialport = "4.2"      # Serial port communication
image = "0.24"          # Image creation
imageproc = "0.23"      # Text drawing on images
rusttype = "0.9"        # Font handling
clap = { version = "4", features = ["derive"] }  # CLI parsing

[dev-dependencies]
mockall = "0.11"        # Mocking for tests
```

---

## File Structure (Proposed)

```
mini_display/
├── Cargo.toml
├── src/
│   ├── main.rs         # CLI entry point
│   ├── lib.rs          # Library exports
│   ├── port.rs         # COM port enumeration & detection
│   ├── connection.rs   # Serial connection handling
│   ├── image.rs        # Image creation & conversion
│   └── protocol.rs     # Display command protocol
└── tests/
    └── integration_tests.rs
```

---

## Notes

- Keep Python implementation as reference during migration
- Port tests alongside implementation
- Focus on macOS first, then cross-platform
