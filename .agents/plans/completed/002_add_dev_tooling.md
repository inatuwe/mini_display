# Plan 002: Add Development Tooling

**Status:** COMPLETED  
**Created:** 2026-01-30  
**Based on:** [rust_dev_tooling_best_practices.md](../research/rust_dev_tooling_best_practices.md)

## Goal

Improve the development feedback loop with simple, effective tooling for coding agents and developers maintaining this codebase.

## Scope

Keep it simple - this is a hobby project. Focus on high-impact, low-effort improvements.

---

## Tasks

### Phase 1: Core Tooling (Must Have)

- [ ] **1.1** Create `.rustfmt.toml` with minimal config
  ```toml
  edition = "2021"
  ```

- [ ] **1.2** Add Clippy lint directive to `src/lib.rs`
  ```rust
  #![warn(clippy::all)]
  ```

- [ ] **1.3** Create `justfile` with common commands
  - `just` / `just --list` - show available commands
  - `just check` - fast type-check
  - `just lint` - run clippy
  - `just fmt` - format code
  - `just test` - run tests
  - `just ci` - full check (fmt + lint + test)
  - `just build` - release build
  - `just run` - run with text
  - `just detect` - detect display

- [ ] **1.4** Update `AGENTS.md` with new commands
  - Add `just` commands to Commands section
  - Document the feedback loop workflow

### Phase 2: CI (Nice to Have)

- [ ] **2.1** Create `.github/workflows/ci.yml`
  - Format check
  - Clippy
  - Tests
  - Build

### Phase 3: Verification

- [ ] **3.1** Run `just ci` to verify all checks pass
- [ ] **3.2** Commit changes with message: "Plan 002: Add dev tooling (justfile, clippy, rustfmt)"

---

## Files to Create/Modify

| File | Action |
|------|--------|
| `.rustfmt.toml` | Create |
| `justfile` | Create |
| `src/lib.rs` | Add clippy lint |
| `AGENTS.md` | Update commands |
| `.github/workflows/ci.yml` | Create (optional) |

---

## Success Criteria

1. `just` shows list of available commands
2. `just ci` runs format, lint, and tests successfully
3. `cargo clippy -- -D warnings` passes
4. AGENTS.md documents the new workflow

---

## Notes

- `just` needs to be installed: `brew install just` or `cargo install just`
- Skip `clippy::pedantic` - too strict for hobby project
- Skip complex CI caching - overkill for small project
