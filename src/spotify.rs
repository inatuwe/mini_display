use std::process::Command;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NowPlaying {
    pub track: String,
    pub artist: String,
    pub is_playing: bool,
}

pub fn get_now_playing() -> Option<NowPlaying> {
    let player_state = run_applescript("tell application \"Spotify\" to player state as string")?;

    let is_playing = player_state == "playing";

    let track = run_applescript("tell application \"Spotify\" to name of current track")?;
    let artist = run_applescript("tell application \"Spotify\" to artist of current track")?;

    Some(NowPlaying {
        track,
        artist,
        is_playing,
    })
}

fn run_applescript(script: &str) -> Option<String> {
    let output = Command::new("osascript")
        .arg("-e")
        .arg(script)
        .output()
        .ok()?;

    if !output.status.success() {
        return None;
    }

    let result = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if result.is_empty() {
        None
    } else {
        Some(result)
    }
}
