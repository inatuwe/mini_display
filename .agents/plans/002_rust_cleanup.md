# Plan 002: Rust Cleanup & Test Migration

**Status:** COMPLETED  
**Priority:** High  
**Created:** January 30, 2026

---

## Objective

Finalize the Rust migration by making Rust the primary source, removing Python code, and migrating tests.

## Goals

1. **Rust as main source** - Move `rust-src/` to `src/`
2. **Remove Python** - Delete Python files and dependencies
3. **Rust tests** - Migrate Python tests to Rust unit tests
4. **Verify functionality** - Ensure no features are missing

---

## Phase 1: Restructure Source Directory (DONE)

- [x] Move `rust-src/*.rs` to `src/`
- [x] Update `Cargo.toml` paths (`lib.rs`, `main.rs`)
- [x] Verify `cargo build` works

## Phase 2: Remove Python Code (DONE)

- [x] Delete `src/` Python modules (after Rust src is moved)
- [x] Delete `tests/` Python tests (after Rust tests exist)
- [x] Delete `detect_display.py`
- [x] Delete `display.py`
- [x] Delete `requirements.txt`
- [x] Delete `__pycache__/` directories

## Phase 3: Migrate Tests to Rust (DONE)

### Port Tests (`port.rs`)
From `test_com_ports.py`:
- [x] `test_list_ports_returns_vec` - list_ports() returns Vec
- [x] `test_find_display_port_returns_option` - find_display_port() behavior
- [x] `test_is_display_connected_returns_bool` - is_display_connected() returns bool
- [x] `test_vid_pid_constants_defined` - VID/PID constants are correct
- [x] `test_port_info_struct` - PortInfo struct fields

### Image Tests (`image.rs`)
From `test_image.py`:
- [x] `test_create_blank_image_dimensions` - 160x80 black image
- [x] `test_create_blank_image_is_black` - blank image is black
- [x] `test_create_text_image_has_content` - text image not all black
- [x] `test_rgb565_black_converts_to_zero` - (0,0,0) → 0x0000
- [x] `test_rgb565_white_converts_correctly` - (255,255,255) → 0xFFFF
- [x] `test_rgb565_red_converts_correctly` - (255,0,0) → 0xF800
- [x] `test_rgb565_green_converts_correctly` - (0,255,0) → 0x07E0
- [x] `test_rgb565_blue_converts_correctly` - (0,0,255) → 0x001F
- [x] `test_rgb565_output_size` - 160×80×2 = 25600 bytes
- [x] `test_display_dimensions` - WIDTH=160, HEIGHT=80

### Protocol Tests (`protocol.rs`)
From `test_serial_comm.py`:
- [x] `test_bitmap_header_structure` - 10-byte header with correct format
- [x] `test_bitmap_header_coordinates` - x0=0, y0=0, x1=159, y1=79
- [x] `test_command_constants` - CMD_SET_BITMAP=0x05, CMD_END=0x0A
- [x] `test_chunk_size` - CHUNK_SIZE=640

## Phase 4: Update Documentation (DONE)

- [x] Update `AGENTS.md` - Remove Python commands, update structure
- [x] Update `README.md` - Remove Python references
- [x] Mark Plan 002 as COMPLETED

---

## Test Mapping: Python → Rust

| Python Test File | Rust Module | Key Tests |
|-----------------|-------------|-----------|
| `test_com_ports.py` | `port.rs` | VID/PID detection, port listing |
| `test_image.py` | `image.rs` | Blank image, RGB565 conversion |
| `test_serial_comm.py` | `protocol.rs` | Header structure |
| `test_detect_display.py` | Integration | CLI `--detect` flag |
| `test_hello_world.py` | Integration | CLI text display |
| `test_error_handling.py` | Error types | Error propagation |

---

## Notes

- Rust tests go in each module file using `#[cfg(test)]` blocks
- Hardware-dependent tests (actual display) remain manual
- Integration tests can use `cargo test --test integration` if needed
