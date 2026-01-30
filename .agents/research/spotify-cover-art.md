# Research: Displaying Spotify Album Cover Art

## Overview

This document explores how to display the album/artist cover image from currently playing Spotify songs on the Display FS V1 (160x80 pixels).

## Spotify Web API

### Relevant Endpoint

**GET /me/player/currently-playing**

- Returns the currently playing track/episode
- Includes album artwork URLs in multiple sizes

### Required OAuth Scope

- `user-read-currently-playing` - Read your currently playing content

### Response Structure (simplified)

```json
{
  "is_playing": true,
  "item": {
    "name": "Track Name",
    "artists": [{"name": "Artist Name"}],
    "album": {
      "name": "Album Name",
      "images": [
        {"url": "https://i.scdn.co/image/...", "height": 640, "width": 640},
        {"url": "https://i.scdn.co/image/...", "height": 300, "width": 300},
        {"url": "https://i.scdn.co/image/...", "height": 64, "width": 64}
      ]
    }
  }
}
```

### Image Sizes Available

- 640x640 (large)
- 300x300 (medium)
- 64x64 (small) ← closest to our 80x80 display height

## Authentication Flow

Spotify requires OAuth 2.0 for accessing user data. Options:

1. **Authorization Code Flow** (recommended for desktop apps)
   - User authorizes once via browser
   - App receives refresh token for persistent access
   - Tokens can be cached locally

2. **Authorization Code with PKCE** (for public clients)
   - Same as above but without client secret
   - More secure for desktop/CLI apps

### Setup Requirements

1. Create Spotify Developer App at <https://developer.spotify.com/dashboard>
2. Get Client ID and Client Secret
3. Set Redirect URI (e.g., `http://localhost:8888/callback`)
4. Request `user-read-currently-playing` scope

## Rust Implementation Options

### Option 1: rspotify (Recommended)

**Crate:** `rspotify`  
**GitHub:** <https://github.com/ramsayleung/rspotify>

Full-featured Spotify Web API SDK:

- Supports all authorization flows
- Has `current_playing_track()` method
- Well-maintained (700+ stars)

```toml
[dependencies]
rspotify = { version = "0.13", features = ["client-reqwest"] }
```

```rust
use rspotify::{prelude::*, scopes, AuthCodeSpotify, Credentials, OAuth};

let creds = Credentials::from_env().unwrap();
let oauth = OAuth::from_env(scopes!("user-read-currently-playing")).unwrap();
let spotify = AuthCodeSpotify::new(creds, oauth);

// After auth...
let playing = spotify.current_playing(None, None::<Vec<_>>).await?;
if let Some(context) = playing {
    if let Some(item) = context.item {
        // Get album images from item.album.images
    }
}
```

### Option 2: spotify-rs

**Crate:** `spotify-rs`  
**Newer alternative, simpler API**

```toml
[dependencies]
spotify-rs = "0.4"
```

### Option 3: Manual HTTP Requests

Use `reqwest` to call the API directly - more control but more work.

## Implementation Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                      CLI Application                         │
├─────────────────────────────────────────────────────────────┤
│  1. Spotify Module                                           │
│     - OAuth authentication (cached tokens)                   │
│     - Poll currently playing track                           │
│     - Fetch album art URL                                    │
├─────────────────────────────────────────────────────────────┤
│  2. Image Processing                                         │
│     - Download image from URL (reqwest)                      │
│     - Resize to 80x80 (image crate)                          │
│     - Convert to RGB565                                      │
├─────────────────────────────────────────────────────────────┤
│  3. Display Module (existing)                                │
│     - Send image to display via serial                       │
└─────────────────────────────────────────────────────────────┘
```

## Workflow

1. **First Run:**
   - Open browser for Spotify OAuth
   - User logs in and authorizes
   - Store refresh token locally (e.g., `~/.config/display-fs/spotify.json`)

2. **Subsequent Runs:**
   - Load cached refresh token
   - Refresh access token automatically

3. **Display Loop:**

   ```
   loop {
       1. Call GET /me/player/currently-playing
       2. If playing and track changed:
          a. Download 64x64 album art (or 300x300 and resize)
          b. Resize to 80x80 for display
          c. Convert to RGB565
          d. Send to display
       3. Sleep 2-5 seconds
   }
   ```

## Display Considerations

- **Display size:** 160x80 pixels
- **Album art:** Square (1:1 aspect ratio)
- **Layout options:**
  1. 80x80 cover on left + track info on right (80x80)
  2. Center 80x80 cover, no text
  3. Full width scaled (stretch to 160x80, distorted)

### Recommended Layout

```text
┌────────────────────────────────────────────────┐
│ ┌──────────┐  Track: Song Name                 │
│ │  Album   │  Artist: Artist Name              │
│ │   Art    │  ▶ ━━━━━━━━━━━━━━ 2:34            │
│ │  80x80   │                                   │
│ └──────────┘                                   │
└────────────────────────────────────────────────┘
        80px                    80px
```

## Dependencies to Add

```toml
[dependencies]
rspotify = { version = "0.13", features = ["client-reqwest"] }
reqwest = { version = "0.12", features = ["blocking"] }
tokio = { version = "1", features = ["rt-multi-thread", "macros"] }
dirs = "5"  # For config directory
```

## Policy Considerations

From Spotify's terms:

- ✅ Display album artwork (must link back to Spotify)
- ✅ Show now playing info
- ❌ Cannot crop/modify album artwork
- ❌ Cannot overlay images on artwork
- ⚠️ Should include Spotify attribution

## Alternative: macOS AppleScript/osascript

For macOS only, can get currently playing without API setup:

```bash
osascript -e 'tell application "Spotify" to get {name, artist, artwork url} of current track'
```

This avoids OAuth complexity but is platform-specific.

## Next Steps

1. Decide on layout (cover only vs cover + text)
2. Create Spotify developer app
3. Implement OAuth flow with token caching
4. Add image download and resize
5. Integrate with existing display code
6. Add polling loop for "now playing" mode
