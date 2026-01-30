# Plan 005: Japanese/CJK Font Support (Feature Flag)

**Status:** COMPLETED  
**Priority:** Medium  
**Complexity:** Low-Medium

## Overview

Add optional Japanese font support via Cargo feature flag. Default build uses DejaVuSans (small), `--features japanese` uses Noto Sans JP (larger but supports CJK).

## Context

- Current font: DejaVuSans.ttf (~750 KB) - no CJK support
- Japanese songs display as boxes (□) on Spotify
- Research: `.agents/research/japanese-character-support.md`
- Noto Sans JP provides full Japanese + Latin coverage (~5 MB)

## Design

```rust
// Cargo.toml
[features]
default = []
japanese = []

// src/image.rs
#[cfg(feature = "japanese")]
const FONT_DATA: &[u8] = include_bytes!("../assets/fonts/NotoSansJP-Regular.otf");

#[cfg(not(feature = "japanese"))]
const FONT_DATA: &[u8] = include_bytes!("../assets/fonts/DejaVuSans.ttf");
```

## Tasks

- [ ] **Task 1: Download Noto Sans JP font**
  - Scope: `assets/fonts/`
  - Depends on: none
  - Acceptance:
    - `assets/fonts/NotoSansJP-Regular.otf` exists
    - Font downloaded from official Google Fonts release
    - DejaVuSans.ttf kept (default font)
    - File size ~5 MB
  - Notes: Download from https://github.com/googlefonts/noto-cjk/releases

- [ ] **Task 2: Add japanese feature flag to Cargo.toml**
  - Scope: `Cargo.toml`
  - Depends on: none
  - Acceptance:
    - `[features]` section exists
    - `japanese = []` feature defined
    - Not in default features
    - `just check` passes

- [ ] **Task 3: Add conditional font compilation in image.rs**
  - Scope: `src/image.rs`
  - Depends on: Task 1, Task 2
  - Acceptance:
    - `#[cfg(feature = "japanese")]` selects NotoSansJP-Regular.otf
    - `#[cfg(not(feature = "japanese"))]` selects DejaVuSans.ttf
    - `just check` passes (both with and without feature)
  - Notes: Use cfg attributes on const FONT_DATA

- [ ] **Task 4: Update justfile with japanese build targets**
  - Scope: `justfile`
  - Depends on: Task 3
  - Acceptance:
    - `just build-jp` builds with japanese feature
    - `just install-jp` installs japanese-enabled binary
    - `just check` and `just check-jp` for both variants
    - Existing commands unchanged (default = no japanese)

- [ ] **Task 5: Add Japanese text rendering test (feature-gated)**
  - Scope: `src/image.rs`
  - Depends on: Task 3
  - Acceptance:
    - New test `test_japanese_text_renders()` with `#[cfg(feature = "japanese")]`
    - Test renders "こんにちは" (Hello in Japanese)
    - Test verifies output differs from blank image
    - `cargo test --features japanese` passes

- [ ] **Task 6: Update README with feature flag documentation**
  - Scope: `README.md`
  - Depends on: Task 4
  - Acceptance:
    - Documents japanese feature flag
    - Shows build commands for both variants
    - Explains binary size difference

- [ ] (manual-verify) **Task 7: Visual verification with display**
  - Scope: N/A
  - Depends on: Task 5
  - Acceptance:
    - `cargo run --features japanese -- "こんにちは世界"` displays correctly
    - `cargo run -- "Hello"` still works (default build)
    - Spotify Japanese songs display correctly with japanese feature

## Build Commands

```bash
# Default build (small, Latin only)
cargo build --release
just build
just install

# Japanese-enabled build (larger, full CJK)
cargo build --release --features japanese
just build-jp
just install-jp

# Test both variants
just check          # Default
just check-jp       # With japanese
cargo test --features japanese
```

## Binary Size Impact

| Build | Font | Size |
|-------|------|------|
| Default | DejaVuSans | ~1.5 MB |
| `--features japanese` | NotoSansJP | ~6.5 MB |

## Future Enhancements (Out of Scope)

- Font fallback system (multiple fonts)
- Lazy font downloading
- Font subsetting for smaller binary
- Support for other CJK languages (Chinese, Korean)
