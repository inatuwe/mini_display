# Plan 004: Multi-Page Text Display with Paging

**Status:** IN-PROGRESS  
**Priority:** High  
**Depends on:** 003 (configurable font size)

## Goal

Extend the CLI to handle long text that doesn't fit on the 160x80 display, automatically splitting it into pages and displaying them sequentially with configurable delays.

## Tasks

- [x] **Task 1: Expose text measurement utilities**
  - Scope: `src/image.rs`, `src/lib.rs`
  - Depends on: none
  - Acceptance:
    - `measure_text()` is public and accessible
    - Add `calculate_max_chars_per_line(font_size: f32) -> usize` function
    - Add `calculate_max_lines(font_size: f32) -> usize` function
    - Functions exported in `lib.rs`
  - Notes: Based on display 160x80 and embedded DejaVuSans font

- [x] **Task 2: Create text splitting module**
  - Scope: `src/text.rs`, `src/lib.rs`
  - Depends on: Task 1
  - Acceptance:
    - New `src/text.rs` module created
    - `split_into_pages(text: &str, font_size: f32) -> Vec<String>` implemented
    - Word-aware splitting (never breaks mid-word)
    - Handles newlines in input text
    - Module exported in `lib.rs`
  - Notes: Use measurement functions from Task 1

- [x] **Task 3: Add delay CLI parameter**
  - Scope: `src/main.rs`
  - Depends on: none
  - Acceptance:
    - `--delay <seconds>` flag added (f32, default: 2.0)
    - Validated as positive number
    - Help text updated
  - Notes: Use clap for argument parsing

- [x] **Task 4: Add loop and speed CLI parameters**
  - Scope: `src/main.rs`
  - Depends on: Task 3
  - Acceptance:
    - `--loop` flag for continuous display
    - `--once` flag for single display (default)
    - `--speed <preset>` with values: slow (4s), normal (2s), fast (1s)
    - Speed presets override delay if both provided
  - Notes: Loop mode runs until Ctrl+C

- [x] **Task 5: Implement multi-page display loop**
  - Scope: `src/main.rs`, `src/lib.rs`
  - Depends on: Task 2, Task 4
  - Acceptance:
    - Text automatically split into pages using `split_into_pages()`
    - Pages displayed sequentially with configured delay
    - Loop mode repeats indefinitely
    - Single page text still works (no delay needed)
  - Notes: Use `std::thread::sleep` for delays

- [x] **Task 6: Add unit tests for text splitting**
  - Scope: `src/text.rs`
  - Depends on: Task 2
  - Acceptance:
    - Test empty string input
    - Test single word that fits
    - Test text requiring multiple pages
    - Test word-boundary splitting (no mid-word breaks)
    - Test input with newlines
    - All tests pass with `just test`
  - Notes: Use inline `#[cfg(test)]` module

- [ ] **Task 7: Integration verification**
  - Scope: `src/main.rs`
  - Depends on: Task 5, Task 6
  - Acceptance:
    - `just ci` passes (fmt + lint + test)
    - Example command works: `cargo run -- "Long text message" --delay 2`
    - Loop mode works: `cargo run -- "Message" --loop`
  - Notes: (manual-verify) Requires display connected for visual check

## Example Usage

```bash
# Display a long message, 2 seconds per page
display-fs "Welcome to the Display FS demo! This text is too long to fit on one screen." --delay 2

# Continuous notification loop
display-fs "Server status: OK | CPU: 45% | Memory: 2.1GB" --loop --delay 5

# Quick status with speed preset
display-fs "BUILD PASSED" --speed fast
```

## Future Enhancements (Not in Scope)

These features are documented for future plans:

- Text alignment options (`--align left|center|right`)
- Text color options (`--color white|red|green|...`)
- Background color (`--bg black|white|...`)
- Clear display command (`--clear`)
- Image file display (`--image path/to/image.png`)
- Verbose/quiet modes (`-v`, `--quiet`)
