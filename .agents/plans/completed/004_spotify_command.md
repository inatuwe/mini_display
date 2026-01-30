# Plan 004: Spotify Command

**Status:** COMPLETED  
**Priority:** Medium  
**Complexity:** Low-Medium

## Overview

Add a dedicated `spotify` subcommand that shows the currently playing Spotify song on the display. This extends beyond the existing `preset spotify` by providing a richer, continuously updating display.

## Context

- A `preset spotify` already exists (line 91-92, 126-129 in main.rs)
- Uses AppleScript: `osascript -e 'tell application "Spotify" to if player state is playing then name of current track else "Not playing"'`
- Only shows track name, no artist info

## Tasks

- [x] **Task 1: Create spotify module with NowPlaying struct**
  - Scope: `src/spotify.rs`, `src/lib.rs`
  - Depends on: none
  - Acceptance:
    - `src/spotify.rs` exists with `NowPlaying` struct (track, artist, is_playing fields)
    - `get_now_playing() -> Option<NowPlaying>` function uses AppleScript to fetch track, artist, player state
    - Module exported from `src/lib.rs`
    - `just check` passes
  - Notes: AppleScript commands: `tell application "Spotify" to name of current track`, `artist of current track`, `player state`

- [x] **Task 2: Add error handling for Spotify states**
  - Scope: `src/spotify.rs`
  - Depends on: Task 1
  - Acceptance:
    - Returns `None` if Spotify not running
    - Returns `NowPlaying { is_playing: false, ... }` if paused
    - Handles AppleScript execution errors gracefully
    - `just check` passes

- [x] **Task 3: Add Spotify subcommand to CLI**
  - Scope: `src/main.rs`
  - Depends on: Task 1
  - Acceptance:
    - `SpotifyArgs` struct with `--loop` / `-l` flag and `--interval` option (default: 2 seconds)
    - `Commands::Spotify(SpotifyArgs)` variant added
    - `run_spotify(args: SpotifyArgs) -> ExitCode` function stub
    - `display-fs spotify --help` works
    - `just check` passes

- [x] **Task 4: Implement single-shot Spotify display**
  - Scope: `src/main.rs`, `src/spotify.rs`
  - Depends on: Task 2, Task 3
  - Acceptance:
    - `display-fs spotify` fetches now playing and sends to display
    - Format: "♪ Track Name\nby Artist" (truncate long names with "...")
    - Shows "Not playing" or "Spotify not running" for error states
    - `just ci` passes

- [x] **Task 5: Implement loop mode**
  - Scope: `src/main.rs`
  - Depends on: Task 4
  - Acceptance:
    - `display-fs spotify --loop` polls every N seconds
    - Only updates display if song changed (compare track+artist)
    - Exits cleanly on Ctrl+C
    - `just ci` passes

- [x] (manual-verify) **Task 6: Manual testing**
  - Scope: N/A
  - Depends on: Task 5
  - Acceptance:
    - Test with Spotify playing, paused, closed
    - Test with very long track/artist names
    - Test loop mode with song changes

## CLI Examples

```bash
display-fs spotify                    # Show current song once
display-fs spotify --loop             # Continuously update
display-fs spotify --loop --interval 5  # Custom interval
```

## Future Enhancements (Out of Scope)

- Album art (would need image support)
- Progress bar
- Playback controls
- Support for other music players (Apple Music, etc.)
