# Research: Japanese/CJK Character Support

## Problem

The current font (DejaVuSans.ttf) does not include Japanese characters (Hiragana, Katakana, Kanji) or other CJK scripts. When Spotify displays Japanese song titles, missing glyphs appear as boxes/tofu (□).

## Root Cause

- **DejaVuSans.ttf** covers Latin, Greek, Cyrillic, and some extended characters
- It does **not** include CJK Unified Ideographs, Hiragana, or Katakana

## Solutions

### Option 1: Switch to Noto Sans CJK (Recommended)

**Pros:**

- Best Unicode coverage (~65,000 glyphs)
- Covers Japanese, Chinese, Korean, Latin, and more
- Multiple weights available
- Actively maintained by Google/Adobe
- SIL Open Font License (free to use/embed)

**Cons:**

- Large file size (~15-20 MB for full CJK)
- Regional subsets available (JP, SC, TC, KR) - Japanese subset is smaller

**Files:**

- `NotoSansCJK-Regular.ttc` (full, ~21 MB)
- `NotoSansJP-Regular.otf` (Japanese only, ~5 MB)

**Download:** <https://github.com/googlefonts/noto-cjk/releases>

### Option 2: M+ FONTS

**Pros:**

- Smaller than Noto (~5-10 MB for Japanese)
- Good Japanese coverage (5,700+ Kanji)
- Harmonious Latin/Japanese integration
- Open source (free use)
- Rounded variants available

**Cons:**

- Less comprehensive than Noto for multi-language

**Download:** <https://github.com/coz-m/MPLUS_FONTS>

### Option 3: Font Fallback (Multiple Fonts)

Load multiple fonts and render with fallback:

1. Primary: DejaVuSans (Latin)
2. Fallback: NotoSansJP (Japanese)

**Pros:**

- Smaller memory footprint if only Latin is used
- Best of both worlds

**Cons:**

- More complex implementation
- Need to detect missing glyphs

### Option 4: Font Subsetting

Create a custom font with only needed characters:

- Use tools like `pyftsubset` or `fonttools`
- Extract only Hiragana, Katakana, and common Kanji

**Pros:**

- Smallest possible file size
- Can target specific use case

**Cons:**

- Missing characters if subset is too small
- Maintenance burden

## Recommendation

### For Spotify Use Case (Japanese Song Titles)

Use **Noto Sans JP** (`NotoSansJP-Regular.otf` or `NotoSansJP-Medium.otf`):

- ~5 MB file size (acceptable for embedded binary)
- Full Japanese coverage (Hiragana, Katakana, Kanji)
- Good Latin coverage
- Optimized for screen display

### Implementation Steps

1. **Download Noto Sans JP**

   ```bash
   curl -L -o NotoSansJP-Regular.otf \
     "https://github.com/googlefonts/noto-cjk/releases/download/Sans2.004/NotoSansJP-Regular.otf"
   ```

2. **Replace font in assets**

   ```
   assets/fonts/NotoSansJP-Regular.otf
   ```

3. **Update Rust code**

   ```rust
   const FONT_DATA: &[u8] = include_bytes!("../assets/fonts/NotoSansJP-Regular.otf");
   ```

4. **Test with Japanese text**

   ```bash
   cargo run -- "こんにちは 世界"  # Hello World in Japanese
   cargo run -- "夜に駆ける"        # "Racing into the Night" (YOASOBI song)
   ```

## Binary Size Impact

| Font                    | Size     | Impact on Binary |
|-------------------------|----------|------------------|
| DejaVuSans.ttf          | ~750 KB  | Current         |
| NotoSansJP-Regular.otf  | ~5 MB    | +4.3 MB         |
| Noto Sans CJK (full)    | ~21 MB   | +20 MB          |

## Alternative: Lazy Font Loading (Future)

Instead of embedding the font, download on first use:

- Check if font exists in `~/.config/display-fs/fonts/`
- If not, download from GitHub releases
- Cache locally for future use

This keeps binary small but requires internet on first run.

## References

- [Noto CJK Fonts](https://github.com/googlefonts/noto-cjk)
- [M+ Fonts](https://github.com/coz-m/MPLUS_FONTS)
- [ab_glyph crate](https://docs.rs/ab_glyph) - Current font rendering library
- [Stack Overflow: CJK Font for embedded system](https://stackoverflow.com/questions/66886362/cjk-font-for-embedded-system)
