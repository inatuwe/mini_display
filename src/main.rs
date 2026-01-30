use clap::{Parser, ValueEnum};
use display_fs::{
    create_text_image, find_display_port, image_to_rgb565_bytes, is_display_connected,
    open_connection, send_image_to_display,
};
use std::process::ExitCode;

#[derive(Parser)]
#[command(name = "display-fs")]
#[command(about = "Display text on WeAct Studio Display FS V1 (0.96 inch)")]
struct Cli {
    /// Text to display (default: "Hello World!")
    #[arg(default_value = "Hello World!")]
    text: String,

    /// Font size in pixels
    #[arg(short = 's', long, default_value = "14")]
    font_size: f32,

    /// Only check if display is connected
    #[arg(short, long)]
    detect: bool,

    /// Delay between pages in seconds (must be positive)
    #[arg(long, default_value = "2.0", value_parser = validate_positive_f32)]
    delay: f32,

    /// Loop display continuously (until Ctrl+C)
    #[arg(long, conflicts_with = "once")]
    r#loop: bool,

    /// Display once only (default behavior)
    #[arg(long, conflicts_with = "loop")]
    once: bool,

    /// Speed preset (overrides --delay if provided)
    #[arg(long, value_enum)]
    speed: Option<SpeedPreset>,
}

#[derive(Clone, Copy, ValueEnum)]
enum SpeedPreset {
    /// 4 seconds between pages
    Slow,
    /// 2 seconds between pages
    Normal,
    /// 1 second between pages
    Fast,
}

impl SpeedPreset {
    pub fn to_delay(self) -> f32 {
        match self {
            SpeedPreset::Slow => 4.0,
            SpeedPreset::Normal => 2.0,
            SpeedPreset::Fast => 1.0,
        }
    }
}

fn validate_positive_f32(s: &str) -> Result<f32, String> {
    let value: f32 = s
        .parse()
        .map_err(|_| format!("'{}' is not a valid number", s))?;
    if value <= 0.0 {
        Err("delay must be a positive number".to_string())
    } else {
        Ok(value)
    }
}

fn main() -> ExitCode {
    let cli = Cli::parse();

    if cli.detect {
        return detect_display();
    }

    // Compute effective delay: speed preset overrides --delay
    let delay = cli.speed.map_or(cli.delay, |s| s.to_delay());
    let loop_mode = cli.r#loop;

    display_text(&cli.text, cli.font_size, delay, loop_mode)
}

fn detect_display() -> ExitCode {
    println!("Looking for Display FS V1...");

    if is_display_connected() {
        if let Some(port) = find_display_port() {
            println!("✓ Found display on {}", port.name);
            println!("  VID: {:04X}, PID: {:04X}", port.vid, port.pid);
            return ExitCode::SUCCESS;
        }
    }

    println!("✗ Display FS V1 not found");
    println!("  Make sure the display is connected via USB-C");
    println!("  and the CH340/CH341 driver is installed.");
    ExitCode::FAILURE
}

fn display_text(text: &str, font_size: f32, _delay: f32, _loop_mode: bool) -> ExitCode {
    println!("Looking for Display FS V1...");

    let port_info = match find_display_port() {
        Some(p) => p,
        None => {
            println!("✗ Display FS V1 not found");
            println!("  Make sure the display is connected via USB-C");
            println!("  and the CH340/CH341 driver is installed.");
            return ExitCode::FAILURE;
        }
    };

    println!("✓ Found display on {}", port_info.name);

    println!(
        "Creating image with text: '{}' (font size: {})",
        text, font_size
    );
    let img = create_text_image(text, font_size);
    let image_data = image_to_rgb565_bytes(&img);
    println!("  Image size: {} bytes", image_data.len());

    println!("Opening connection to {}...", port_info.name);
    let mut connection = match open_connection(&port_info) {
        Ok(c) => c,
        Err(e) => {
            println!("✗ Failed to open connection: {}", e);
            return ExitCode::FAILURE;
        }
    };
    println!("✓ Connection opened");

    println!("Sending image to display...");
    match send_image_to_display(&mut connection, &image_data) {
        Ok(()) => {
            println!("✓ Image sent successfully!");
            println!();
            println!("'{}' should now be displayed!", text);
            ExitCode::SUCCESS
        }
        Err(e) => {
            println!("✗ Failed to send image: {}", e);
            ExitCode::FAILURE
        }
    }
}
