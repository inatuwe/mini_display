#![warn(clippy::all)]

pub mod image;
pub mod port;
pub mod protocol;
pub mod text;

pub use image::{
    calculate_max_chars_per_line, calculate_max_lines, create_text_image, image_to_rgb565_bytes,
    measure_text_with_font_size, DISPLAY_HEIGHT, DISPLAY_WIDTH,
};
pub use port::{find_display_port, is_display_connected, open_connection, PortInfo};
pub use protocol::send_image_to_display;
pub use text::split_into_pages;
