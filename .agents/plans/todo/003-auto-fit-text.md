# Plan 003: Auto-Fit Text Display

**Status:** TODO

## Goal

Display text at the largest possible font size that fits the 160×80 pixel screen, maximizing readability on the small display.

## Current State

- Fixed font size via `--font-size` flag (default: 14px)
- `measure_text_with_font_size()` already measures text dimensions
- `calculate_max_chars_per_line()` and `calculate_max_lines()` exist but aren't used for auto-fitting

## Algorithm

Binary search to find the largest font size (8-80px) where text fits within display bounds:

1. Measure text dimensions at candidate font size
2. Check if width ≤ 156px and height ≤ 76px (4px padding)
3. Binary search converges in ~7 iterations

## Tasks

- [ ] **Task 1: Add measure_multiline_text helper**
  - Scope: `src/image.rs`
  - Depends on: none
  - Acceptance:
    - Function `measure_multiline_text(text: &str, font_size: f32) -> (u32, u32)` exists
    - Returns (max_line_width, total_height) for multi-line text
    - Handles single-line text correctly
  - Notes: Reuse existing `measure_text()` for each line, use `scaled_font.height()` for line height

- [ ] **Task 2: Add calculate_auto_fit_size function**
  - Scope: `src/image.rs`
  - Depends on: Task 1
  - Acceptance:
    - Function `pub fn calculate_auto_fit_size(text: &str) -> f32` exists
    - Uses binary search between MIN_SIZE=8.0 and MAX_SIZE=80.0
    - Returns largest font size where text fits in 156×76 pixels
    - Returns MIN_SIZE for empty string
  - Notes: Binary search with 0.5 precision tolerance

- [ ] **Task 3: Export calculate_auto_fit_size in lib.rs**
  - Scope: `src/lib.rs`
  - Depends on: Task 2
  - Acceptance:
    - `calculate_auto_fit_size` is exported from `lib.rs`
    - Compiles with `just check`

- [ ] **Task 4: Add --auto flag to DisplayOptions**
  - Scope: `src/main.rs`
  - Depends on: Task 3
  - Acceptance:
    - `DisplayOptions` struct has `auto: bool` field with `-a, --auto` flags
    - Flag defaults to false
    - Help text says "Auto-fit text to largest readable size"
  - Notes: Add after `font_size` field

- [ ] **Task 5: Integrate auto-fit in display_text function**
  - Scope: `src/main.rs`
  - Depends on: Task 4
  - Acceptance:
    - When `--auto` is set, font size is computed via `calculate_auto_fit_size()`
    - Original `--font-size` is ignored when `--auto` is set
    - Console output shows computed font size (e.g., "Auto-fit font size: 32.0")
  - Notes: Import `calculate_auto_fit_size` from crate, apply before `split_into_pages`

- [ ] **Task 6: Integrate auto-fit in other commands**
  - Scope: `src/main.rs`
  - Depends on: Task 5
  - Acceptance:
    - `run_preset()` respects `--auto` flag
    - `run_demo()` respects `--auto` flag
    - `run_spotify()` respects `--auto` flag
  - Notes: Use helper function to get effective font size

- [ ] **Task 7: Add tests for auto-fit**
  - Scope: `src/image.rs`
  - Depends on: Task 2
  - Acceptance:
    - Test: single char "X" returns size > 40.0
    - Test: long text "Hello World!" returns size < 30.0
    - Test: empty string returns MIN_SIZE (8.0)
    - Test: multi-line text returns smaller size than single line
    - All tests pass with `just test`

- [ ] **Task 8: Update README with auto-fit feature**
  - Scope: `README.md`
  - Depends on: Task 6
  - Acceptance:
    - Usage section shows `--auto` flag
    - Example: `display-fs show --auto "Hi"` with explanation
    - Feature description explains auto-sizing behavior

## Usage Examples

```bash
# Auto-fit single word (will be large)
display-fs show --auto "Hi"

# Auto-fit longer text (will be smaller)
display-fs show --auto "Hello World"

# Works with presets
display-fs preset clock --auto
```

## Edge Cases

| Input | Expected Behavior |
|-------|-------------------|
| Empty string | Use minimum font size (8px) |
| Single char "X" | Max size ~70-80px (fills screen) |
| Very long text | Minimum size 8px, may need pagination |
| Multi-line | Size based on widest line and total height |
