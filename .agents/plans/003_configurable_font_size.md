# Plan 003: Configurable Font Size

**Status:** TODO

## Problem

When displaying text on the Display FS V1, the font appears small because:

1. The `draw_text()` function falls back to PIL's default bitmap font when `arial.ttf` is not found
2. PIL's `ImageFont.load_default()` returns a fixed-size bitmap font that ignores the `font_size` parameter
3. No CLI option exists to specify font size

## Goals

1. Bundle a scalable TrueType font with the project
2. Ensure `font_size` parameter actually controls text size
3. Add CLI argument `--font-size` to `hello_world.py`

## Implementation

### Task 1: Add bundled font

- [ ] Create `assets/fonts/` directory
- [ ] Add a free TrueType font (e.g., DejaVu Sans, Open Sans, or Roboto)
- [ ] Update `.gitignore` if needed

### Task 2: Update `src/image.py`

- [ ] Modify `draw_text()` to use bundled font path
- [ ] Add font path resolution (relative to package)
- [ ] Keep fallback to default font with warning

```python
import os

FONT_PATH = os.path.join(os.path.dirname(__file__), "..", "assets", "fonts", "DejaVuSans.ttf")

def draw_text(image, text, position=None, font_size=12, color=(255, 255, 255)):
    try:
        font = ImageFont.truetype(FONT_PATH, font_size)
    except (IOError, OSError):
        print(f"Warning: Font not found at {FONT_PATH}, using default (size ignored)")
        font = ImageFont.load_default()
```

### Task 3: Add CLI argument to `hello_world.py`

- [ ] Add `argparse` for command-line options
- [ ] Add `--font-size` / `-s` argument (default: 14)
- [ ] Add `--text` / `-t` argument for custom text
- [ ] Pass font size to `create_hello_world_image()` or create new function

```python
import argparse

parser = argparse.ArgumentParser(description="Display text on Display FS V1")
parser.add_argument("-s", "--font-size", type=int, default=14, help="Font size (default: 14)")
parser.add_argument("-t", "--text", default="Hello World!", help="Text to display")
```

### Task 4: Update `create_hello_world_image()`

- [ ] Add `text` and `font_size` parameters
- [ ] Or create new `create_text_image(text, font_size)` function

### Task 5: Add tests

- [ ] Test font loading with bundled font
- [ ] Test different font sizes render different image sizes
- [ ] Test CLI argument parsing

## Recommended Font Sizes

Given the 160x80 display:

- **Small:** 10-12px (multi-line text)
- **Medium:** 14-16px (single line, ~10 chars)
- **Large:** 20-24px (short text, ~5-6 chars)
- **XL:** 30-40px (1-2 chars, icons)

## Files to Modify

- `src/image.py` - Font loading and text rendering
- `hello_world.py` - CLI arguments
- `assets/fonts/` - New directory with bundled font

## Testing

```bash
# Test different sizes
python hello_world.py --font-size 10 --text "Small text"
python hello_world.py --font-size 20 --text "Big"
python hello_world.py --font-size 30 --text "XL"
```
