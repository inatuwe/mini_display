# Rust Development Tooling Best Practices

**Status:** Research Complete  
**Date:** 2026-01-30  
**Goal:** Improve feedback loop for coding agents maintaining this codebase

## Summary

This research covers simple, effective tooling for hobby Rust projects. Focus is on:

- Fast feedback loops with `cargo check` and `clippy`
- Consistent code style with `rustfmt`
- Task automation with `just` (simpler than Makefile)
- Optional: lightweight CI with GitHub Actions

---

## 1. Core Cargo Commands

| Command | Purpose | Speed |
|---------|---------|-------|
| `cargo check` | Type-check without building binary | **Fastest** |
| `cargo clippy` | Lint for common mistakes | Fast |
| `cargo fmt` | Format code | Instant |
| `cargo test` | Run unit tests | Varies |
| `cargo build` | Build debug binary | Slower |
| `cargo build --release` | Optimized binary | Slowest |

**Recommendation:** Use `cargo check` frequently during development. It's much faster than `cargo build` because it skips codegen.

---

## 2. Clippy Configuration

Clippy has lint categories with different default levels:

| Category | Description | Default |
|----------|-------------|---------|
| `clippy::all` | Correctness, suspicious, style, complexity, perf | **warn** |
| `clippy::pedantic` | Stricter, more opinionated lints | allow |
| `clippy::nursery` | Experimental lints | allow |
| `clippy::cargo` | Cargo.toml checks | allow |

### Simple Approach (Recommended for Hobby Projects)

Add to `src/lib.rs`:

```rust
#![warn(clippy::all)]
```

Run with:

```bash
cargo clippy -- -D warnings  # Fail on any warning
```

### Stricter Approach (Optional)

```rust
#![warn(clippy::all, clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]  # Common false positive
```

---

## 3. Rustfmt Configuration

Create `.rustfmt.toml` in project root for consistent formatting:

```toml
# Minimal config - use defaults, they're good
edition = "2021"
```

Or with some common preferences:

```toml
edition = "2021"
max_width = 100
use_small_heuristics = "Default"
```

Run with:

```bash
cargo fmt           # Format all files
cargo fmt -- --check  # Check without modifying (for CI)
```

---

## 4. Task Runner: `just` vs Makefile

### Why `just` over Make?

| Feature | Make | just |
|---------|------|------|
| List available commands | Manual | `just --list` |
| Tab vs space | Tabs required | Either works |
| `.PHONY` needed | Yes | No (command runner, not build system) |
| Variable syntax | `$(VAR)`, `?=`, `:=` | `{{var}}`, `:=` only |
| Cross-platform | Mostly | Yes (written in Rust) |
| Installation | Pre-installed | `brew install just` or `cargo install just` |

### Example `justfile` for this project

```just
# List available commands
default:
    @just --list

# Check code compiles (fast)
check:
    cargo check

# Run all lints
lint:
    cargo clippy -- -D warnings

# Format code
fmt:
    cargo fmt

# Run tests
test:
    cargo test

# Full check: fmt, lint, test
ci: fmt lint test

# Build release binary
build:
    cargo build --release

# Run with text
run text="Hello World!":
    cargo run -- "{{text}}"

# Detect display
detect:
    cargo run -- --detect
```

Usage:

```bash
just          # Show available commands
just check    # Fast type-check
just ci       # Full check before commit
just run "Hi" # Run with custom text
```

---

## 5. Simple GitHub Actions CI (Optional)

Create `.github/workflows/ci.yml`:

```yaml
name: CI

on:
  push:
    branches: [main]
  pull_request:
    branches: [main]

jobs:
  check:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
        with:
          components: clippy, rustfmt
      
      - name: Check formatting
        run: cargo fmt -- --check
      
      - name: Clippy
        run: cargo clippy -- -D warnings
      
      - name: Test
        run: cargo test
      
      - name: Build
        run: cargo build --release
```

This is minimal and fast (~2-3 minutes).

---

## 6. Recommended Implementation

For this hobby project, keep it simple:

### Must Have (High Impact, Low Effort)

1. **Add `.rustfmt.toml`** - ensures consistent formatting
2. **Add `justfile`** - easy task automation
3. **Add Clippy lint to `lib.rs`** - catch common mistakes

### Nice to Have

1. **GitHub Actions CI** - catches issues on push
2. **Pre-commit hook** - runs `just ci` before commits

### Skip for Now

- `clippy::pedantic` - too strict for hobby projects
- Complex CI with caching - overkill for small projects
- `cargo-nextest` - standard `cargo test` is fine

---

## 7. Updated Commands for AGENTS.md

```bash
# Quick check (fastest feedback)
just check

# Full check before commit
just ci

# Or manually:
cargo fmt
cargo clippy -- -D warnings
cargo test
cargo build --release
```

---

## Sources

- [Shuttle: Setting up effective CI/CD for Rust](https://www.shuttle.dev/blog/2025/01/23/setup-rust-ci-cd)
- [rust-clippy GitHub](https://github.com/rust-lang/rust-clippy)
- [just command runner](https://github.com/casey/just)
- [dtolnay/rust-toolchain](https://github.com/dtolnay/rust-toolchain) - recommended for CI
- [Cargo Book: Commands](https://doc.rust-lang.org/cargo/commands/index.html)
