# Display FS V1 - Extended Use Cases Research

Research on creative and practical use cases for small USB-connected displays like the Display FS V1 (160x80).

## Developer Productivity Tools

### Build & CI Status Monitor

Display real-time build status from CI/CD pipelines:

```bash
# Watch GitHub Actions status
display-fs "$(gh run list --limit 1 --json status -q '.[0].status')" --loop --delay 30
```

- Green for passing, red for failing builds
- Show commit hash of last deployment
- Display test coverage percentage

### Git Status Widget

Show current branch and uncommitted changes:

```bash
display-fs "$(git branch --show-current): $(git status --short | wc -l) changes"
```

### Pomodoro Timer

Visual work/break timer:

```bash
display-fs "WORK: 25:00" --color red
sleep 1500
display-fs "BREAK: 5:00" --color green
```

## System Monitoring

### Resource Monitor

Display CPU, memory, disk usage:

```bash
display-fs "CPU: $(top -l 1 | grep 'CPU usage' | awk '{print $3}')" --loop --delay 5
```

### Network Status

Show IP address, connection status, bandwidth:

```bash
display-fs "IP: $(ipconfig getifaddr en0)" --delay 10 --loop
```

### Docker Container Status

Monitor running containers:

```bash
display-fs "Containers: $(docker ps -q | wc -l) running"
```

## Notifications & Alerts

### Email/Message Counter

Display unread message counts:

```bash
# Integration with notmuch, mu, or IMAP
display-fs "📧 12 unread" --color yellow
```

### Calendar Reminders

Show upcoming meetings:

```bash
display-fs "Meeting: 10:00 AM - Standup" --delay 30 --loop
```

### Slack/Discord Status

Display DM counts or mentions:

```bash
display-fs "Slack: 3 DMs" --color cyan
```

## IoT & Smart Home

### Weather Display

Current conditions and forecast:

```bash
display-fs "☀️ 72°F Clear" --delay 300 --loop
```

### Home Automation Status

Smart device states:

```bash
display-fs "Lights: ON | Door: Locked"
```

### Sensor Readings

Temperature, humidity from IoT sensors:

```bash
display-fs "Temp: 68°F | Hum: 45%"
```

## Creative & Personal

### Now Playing

Display current Spotify/music track:

```bash
display-fs "♪ $(osascript -e 'tell application \"Spotify\" to name of current track')"
```

### Motivational Quotes

Random quote display:

```bash
display-fs "$(fortune -s)" --delay 60 --loop
```

### Custom Desk Badge

Name/title display:

```bash
display-fs "Marius | Software Engineer" --color cyan
```

## Trading & Finance

### Stock Ticker

Display stock prices:

```bash
display-fs "AAPL: $185.42 ▲2.3%" --color green --loop --delay 60
```

### Crypto Prices

Bitcoin/Ethereum prices:

```bash
display-fs "BTC: $43,250 | ETH: $2,340"
```

## Gaming & Streaming

### Stream Stats

Viewer count, chat activity:

```bash
display-fs "👁 1,234 viewers | 💬 45/min"
```

### Game Server Status

Minecraft/game server player count:

```bash
display-fs "Server: 12/20 players"
```

## Integration Ideas

### Webhook Receiver

Small HTTP server that updates display:

```rust
// Future: display-fs --server 8080
// POST /display with text payload
```

### MQTT Subscriber

Subscribe to MQTT topics for IoT integration:

```bash
# Future: display-fs --mqtt broker.local --topic home/status
```

### Pipe Support

Accept text from stdin:

```bash
echo "Hello World" | display-fs
tail -f /var/log/app.log | display-fs --follow
```

### Watch Mode

Re-run command periodically:

```bash
display-fs --watch 5 "date '+%H:%M:%S'"
```

## Hardware Extensions

### Multiple Displays

Support for multiple connected displays:

```bash
display-fs "Display 1" --device /dev/tty.usbserial-1
display-fs "Display 2" --device /dev/tty.usbserial-2
```

### Brightness Control

If hardware supports it:

```bash
display-fs "Text" --brightness 50
```

### Screen Rotation

Portrait/landscape modes:

```bash
display-fs "Rotated" --rotate 90
```

## Command Composition Examples

### Full Status Bar Script

```bash
#!/bin/bash
while true; do
    TIME=$(date '+%H:%M')
    CPU=$(top -l 1 | grep 'CPU usage' | awk '{print int($3)}')
    MEM=$(memory_pressure | grep 'System-wide memory free' | awk '{print $NF}')
    display-fs "$TIME | CPU:$CPU% | $MEM free" --quiet
    sleep 30
done
```

### Git Commit Watcher

```bash
#!/bin/bash
LAST_COMMIT=""
while true; do
    COMMIT=$(git log -1 --format="%h %s" 2>/dev/null | cut -c1-25)
    if [ "$COMMIT" != "$LAST_COMMIT" ]; then
        display-fs "New: $COMMIT" --color green
        LAST_COMMIT="$COMMIT"
    fi
    sleep 10
done
```

## Future Feature Ideas

1. **Animation Support** - Simple frame-based animations
2. **Widget System** - Composable display layouts (clock + temp + status)
3. **Touch Integration** - If display supports touch input
4. **Audio Alerts** - Pair visual with audio notifications
5. **Mobile App** - Phone app to send text to display
6. **Home Assistant Integration** - Native HA component
7. **Scriptable Widgets** - Lua/Python for custom logic
8. **QR Code Display** - Generate and show QR codes
9. **Progress Bars** - Visual progress indicators
10. **Emoji/Icon Support** - Small graphics library

## References

- Adafruit small displays: <https://www.adafruit.com/category/63>
- USB LCD projects on Hackaday
- r/homelab mini displays
- Developer desk setups with status monitors
