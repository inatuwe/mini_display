use clap::{Parser, Subcommand, ValueEnum};
use display_fs::{
    create_text_image, find_display_port, image_to_rgb565_bytes, is_display_connected,
    open_connection, send_image_to_display, split_into_pages,
};
use std::process::{Command, ExitCode};
use std::thread;
use std::time::Duration;

#[derive(Parser)]
#[command(name = "display-fs")]
#[command(about = "Display text on WeAct Studio Display FS V1 (0.96 inch)")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Run a built-in preset demo
    Preset {
        /// Preset name to run
        #[arg(value_enum)]
        name: PresetName,
    },
    /// List all available presets
    Presets,
    /// Demo mode: cycle through all presets in a loop
    Demo {
        /// Delay between presets in seconds
        #[arg(short, long, default_value = "5")]
        delay: f32,
    },
    /// Display text on the screen (default command)
    Show(ShowArgs),
}

#[derive(clap::Args)]
struct ShowArgs {
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

#[derive(Clone, Copy, Debug, ValueEnum)]
enum PresetName {
    /// Current time (HH:MM:SS)
    Clock,
    /// Current date and time
    DateTime,
    /// System uptime
    Uptime,
    /// Current git branch and status
    Git,
    /// Local IP address
    Ip,
    /// Username and hostname
    Whoami,
    /// Current working directory
    Pwd,
    /// CPU usage percentage (macOS)
    Cpu,
    /// Memory pressure (macOS)
    Memory,
    /// Docker container count
    Docker,
    /// Now playing from Spotify (macOS)
    Spotify,
    /// Random fortune cookie
    Fortune,
}

impl PresetName {
    /// Returns (description, shell command)
    pub fn info(self) -> (&'static str, &'static str) {
        match self {
            PresetName::Clock => ("Current time", "date '+%H:%M:%S'"),
            PresetName::DateTime => ("Date and time", "date '+%Y-%m-%d %H:%M'"),
            PresetName::Uptime => ("System uptime", "uptime | awk '{print $3, $4}' | sed 's/,$//'"),
            PresetName::Git => (
                "Git branch & status",
                "echo \"$(git branch --show-current 2>/dev/null || echo 'no repo'): $(git status --short 2>/dev/null | wc -l | tr -d ' ') changes\"",
            ),
            PresetName::Ip => (
                "Local IP address",
                "echo \"IP: $(ipconfig getifaddr en0 2>/dev/null || hostname -I 2>/dev/null | awk '{print $1}' || echo 'N/A')\"",
            ),
            PresetName::Whoami => ("Username @ hostname", "echo \"$(whoami)@$(hostname -s)\""),
            PresetName::Pwd => ("Current directory", "basename \"$PWD\""),
            PresetName::Cpu => (
                "CPU usage (macOS)",
                "top -l 1 -n 0 | grep 'CPU usage' | awk '{print \"CPU: \" $3}'",
            ),
            PresetName::Memory => (
                "Memory pressure (macOS)",
                "memory_pressure 2>/dev/null | grep 'System-wide' | awk '{print \"Mem: \" $NF}' || echo 'Mem: N/A'",
            ),
            PresetName::Docker => (
                "Docker containers",
                "echo \"Docker: $(docker ps -q 2>/dev/null | wc -l | tr -d ' ') running\"",
            ),
            PresetName::Spotify => (
                "Spotify now playing (macOS)",
                "osascript -e 'tell application \"Spotify\" to if player state is playing then name of current track else \"Not playing\"' 2>/dev/null || echo 'Spotify N/A'",
            ),
            PresetName::Fortune => ("Random fortune", "fortune -s 2>/dev/null || echo 'Install fortune'"),
        }
    }

    pub fn run_command(self) -> String {
        let (_, cmd) = self.info();
        match Command::new("sh").arg("-c").arg(cmd).output() {
            Ok(output) => {
                let result = String::from_utf8_lossy(&output.stdout).trim().to_string();
                if result.is_empty() {
                    String::from_utf8_lossy(&output.stderr).trim().to_string()
                } else {
                    result
                }
            }
            Err(e) => format!("Error: {}", e),
        }
    }
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

    match cli.command {
        Some(Commands::Preset { name }) => run_preset(name),
        Some(Commands::Presets) => list_presets(),
        Some(Commands::Demo { delay }) => run_demo(delay),
        Some(Commands::Show(args)) => run_show(args),
        None => {
            // Default: show help
            use clap::CommandFactory;
            Cli::command().print_help().ok();
            println!();
            ExitCode::SUCCESS
        }
    }
}

fn run_show(args: ShowArgs) -> ExitCode {
    if args.detect {
        return detect_display();
    }

    // Compute effective delay: speed preset overrides --delay
    let delay = args.speed.map_or(args.delay, |s| s.to_delay());
    let loop_mode = args.r#loop;

    display_text(&args.text, args.font_size, delay, loop_mode)
}

fn list_presets() -> ExitCode {
    println!("Available presets:\n");

    for preset in ALL_PRESETS {
        let (desc, _) = preset.info();
        let name = format!("{:?}", preset).to_lowercase();
        println!("  {:12} - {}", name, desc);
    }

    println!("\nUsage:");
    println!("  display-fs preset <NAME>    Run a single preset");
    println!("  display-fs demo             Cycle through all presets");
    println!("\nExamples:");
    println!("  display-fs preset clock");
    println!("  display-fs demo --delay 3");
    ExitCode::SUCCESS
}

fn run_preset(name: PresetName) -> ExitCode {
    let (desc, _) = name.info();
    println!("Running preset: {}", desc);

    let text = name.run_command();
    println!("Output: {}", text);

    display_text(&text, 14.0, 2.0, false)
}

const ALL_PRESETS: [PresetName; 12] = [
    PresetName::Clock,
    PresetName::DateTime,
    PresetName::Uptime,
    PresetName::Git,
    PresetName::Ip,
    PresetName::Whoami,
    PresetName::Pwd,
    PresetName::Cpu,
    PresetName::Memory,
    PresetName::Docker,
    PresetName::Spotify,
    PresetName::Fortune,
];

fn run_demo(delay: f32) -> ExitCode {
    println!("Demo mode: cycling through all presets (Ctrl+C to stop)");
    println!("Delay: {}s between presets\n", delay);

    let port_info = match find_display_port() {
        Some(p) => p,
        None => {
            println!("✗ Display FS V1 not found");
            return ExitCode::FAILURE;
        }
    };

    println!("✓ Found display on {}", port_info.name);

    let mut connection = match open_connection(&port_info) {
        Ok(c) => c,
        Err(e) => {
            println!("✗ Failed to open connection: {}", e);
            return ExitCode::FAILURE;
        }
    };

    let delay_duration = Duration::from_secs_f32(delay);

    loop {
        for preset in ALL_PRESETS {
            let (desc, _) = preset.info();
            let text = preset.run_command();
            println!("[{}] {}", desc, text);

            let img = create_text_image(&text, 14.0);
            let image_data = image_to_rgb565_bytes(&img);

            if let Err(e) = send_image_to_display(&mut connection, &image_data) {
                println!("✗ Failed to send image: {}", e);
                return ExitCode::FAILURE;
            }

            thread::sleep(delay_duration);
        }
    }
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

fn display_text(text: &str, font_size: f32, delay: f32, loop_mode: bool) -> ExitCode {
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

    // Split text into pages
    let pages = split_into_pages(text, font_size);
    let pages = if pages.is_empty() {
        vec![text.to_string()]
    } else {
        pages
    };

    let page_count = pages.len();
    let needs_delay = page_count > 1 || loop_mode;

    println!(
        "Text split into {} page(s) (font size: {})",
        page_count, font_size
    );

    println!("Opening connection to {}...", port_info.name);
    let mut connection = match open_connection(&port_info) {
        Ok(c) => c,
        Err(e) => {
            println!("✗ Failed to open connection: {}", e);
            return ExitCode::FAILURE;
        }
    };
    println!("✓ Connection opened");

    let delay_duration = Duration::from_secs_f32(delay);

    loop {
        for (i, page) in pages.iter().enumerate() {
            if page_count > 1 {
                println!("Displaying page {}/{}...", i + 1, page_count);
            }

            let img = create_text_image(page, font_size);
            let image_data = image_to_rgb565_bytes(&img);

            match send_image_to_display(&mut connection, &image_data) {
                Ok(()) => {
                    if page_count == 1 && !loop_mode {
                        println!("✓ Image sent successfully!");
                        println!();
                        println!("'{}' should now be displayed!", text);
                    }
                }
                Err(e) => {
                    println!("✗ Failed to send image: {}", e);
                    return ExitCode::FAILURE;
                }
            }

            // Wait between pages (but not after the last page in single-run mode)
            if needs_delay {
                let is_last_page = i == page_count - 1;
                if !is_last_page || loop_mode {
                    thread::sleep(delay_duration);
                }
            }
        }

        if !loop_mode {
            break;
        }
        // Loop mode: continue displaying pages indefinitely
    }

    ExitCode::SUCCESS
}
