# Plan 004: Multi-Page Text Display with Paging

**Status:** TODO  
**Priority:** High  
**Depends on:** 003 (configurable font size)

## Goal

Extend the CLI to handle long text that doesn't fit on the 160x80 display, automatically splitting it into pages and displaying them sequentially with configurable delays.

## Current Behavior

- Text is rendered centered on a single 160x80 frame
- Long text gets clipped or overflows
- No support for multi-line or multi-page content

## Proposed Features

### Core Features

1. **Text Splitting** - Intelligently split text into display-sized chunks
   - Word-aware splitting (never break words)
   - Calculate max characters per line based on font size
   - Support multi-line display (2-3 lines depending on font size)

2. **Page Display Loop** - Show pages sequentially
   - Configurable delay between pages (default: 2-3 seconds)
   - Option to loop continuously or display once

3. **CLI Parameters**

   ```bash
   display-fs "Long text that spans multiple pages"
   display-fs "Long text" --delay 3        # 3 seconds between pages
   display-fs "Long text" --loop           # Repeat forever
   display-fs "Long text" --once           # Show once (default)
   display-fs "Long text" --speed slow     # Presets: slow/normal/fast
   ```

### Additional CLI Enhancements

1. **Text Alignment Options**

   ```bash
   display-fs "Text" --align left|center|right
   ```

2. **Text Color Options**

   ```bash
   display-fs "Text" --color white|red|green|blue|yellow|cyan
   display-fs "Text" --color "#FF5500"     # Hex color
   ```

3. **Background Color**

   ```bash
   display-fs "Text" --bg black|white|...
   ```

4. **Clear Display**

   ```bash
   display-fs --clear                      # Show blank screen
   ```

5. **Display Image File**

   ```bash
   display-fs --image path/to/image.png    # Display an image
   ```

6. **Verbose/Quiet Modes**

   ```bash
   display-fs "Text" --quiet               # No status output
   display-fs "Text" -v                    # Verbose output
   ```

## Implementation Plan

### Phase 1: Text Measurement & Splitting

1. **Expose text measurement** in `image.rs`
   - Make `measure_text()` public
   - Add `calculate_max_chars_per_line(font_size) -> usize`
   - Add `calculate_max_lines(font_size) -> usize`

2. **Create text splitter** in new `src/text.rs`
   - `split_into_pages(text: &str, font_size: f32) -> Vec<String>`
   - Word-aware splitting
   - Handle newlines in input

### Phase 2: Multi-Page Display Loop

1. **Update `display_text()` function**
   - Accept delay parameter
   - Loop through pages
   - Send each page to display with delay

2. **Add CLI parameters**
   - `--delay <seconds>` (f32, default: 2.0)
   - `--loop` flag for continuous display
   - `--speed <preset>` for quick configuration

### Phase 3: Color & Alignment (Optional)

1. **Update `create_text_image()` signature**
   - Add color parameter: `Rgb<u8>`
   - Add background color parameter
   - Add alignment enum: `Left | Center | Right`

2. **Add CLI color flags**
   - Parse color names and hex codes
   - Validate input

### Phase 4: Image Display (Optional)

1. **Add image loading** in `image.rs`
   - Load PNG/JPEG
   - Resize to 160x80
   - Convert to RGB565

2. **Add `--image` CLI flag**

## Files to Modify

| File | Changes |
|------|---------|
| `src/main.rs` | Add CLI args, multi-page loop, color parsing |
| `src/image.rs` | Public measure_text, add color/alignment params |
| `src/text.rs` | NEW: Text splitting logic |
| `src/lib.rs` | Export new types and functions |

## Testing Strategy

- Unit tests for text splitting edge cases
- Unit tests for color parsing
- Integration test with mock display (if feasible)

## Success Criteria

- [ ] Long text displays across multiple pages automatically
- [ ] Delay between pages is configurable
- [ ] Words are never split mid-word
- [ ] Loop mode works correctly
- [ ] Color options work (Phase 3)
- [ ] Image display works (Phase 4)

## Example Usage

```bash
# Display a long message, 2 seconds per page
display-fs "Welcome to the Display FS demo! This text is too long to fit on one screen." --delay 2

# Continuous notification loop
display-fs "Server status: OK | CPU: 45% | Memory: 2.1GB" --loop --delay 5

# Quick status with color
display-fs "BUILD PASSED" --color green --delay 3

# Error notification
display-fs "ERROR: Connection failed" --color red --loop --delay 1
```
