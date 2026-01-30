# Rust First-Time Setup Guide

**Official Docs:** https://www.rust-lang.org/tools/install

---

## Quick Install

### macOS / Linux / WSL

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Follow the on-screen instructions (press Enter for defaults).

After installation, restart your terminal or run:

```bash
source "$HOME/.cargo/env"
```

### Windows

1. Download [rustup-init.exe (x64)](https://static.rust-lang.org/rustup/dist/x86_64-pc-windows-msvc/rustup-init.exe)
2. Run the installer and follow prompts
3. **Required:** Install [Visual Studio C++ Build Tools](https://rust-lang.github.io/rustup/installation/windows-msvc.html) when prompted

---

## Verify Installation

```bash
rustc --version
cargo --version
```

Expected output:

```text
rustc 1.XX.X (hash date)
cargo 1.XX.X (hash date)
```

---

## Essential Commands

| Command | Description |
|---------|-------------|
| `cargo build` | Compile debug build |
| `cargo build --release` | Compile optimized release build |
| `cargo run` | Build and run |
| `cargo run -- args` | Run with arguments |
| `cargo check` | Fast syntax/type check (no compile) |
| `cargo test` | Run tests |
| `rustup update` | Update Rust toolchain |

---

## Project Structure

After `cargo build`, you'll have:

```text
project/
├── Cargo.toml      # Project manifest (dependencies, metadata)
├── Cargo.lock      # Locked dependency versions (auto-generated)
├── src/            # Source files
└── target/         # Build artifacts
    ├── debug/      # Debug build output
    └── release/    # Release build output (with --release)
```

---

## Troubleshooting

| Issue | Solution |
|-------|----------|
| `cargo: command not found` | Restart terminal or run `source ~/.cargo/env` |
| Linker errors on macOS | Install Xcode CLI: `xcode-select --install` |
| Linker errors on Linux | Install build tools: `sudo apt install build-essential` |
| Windows MSVC errors | Install VS C++ Build Tools (link above) |

---

## Resources

- **Official Docs:** https://doc.rust-lang.org/book/
- **Rustup Manual:** https://rust-lang.github.io/rustup/
- **Cargo Book:** https://doc.rust-lang.org/cargo/

**Last Updated:** January 30, 2026
