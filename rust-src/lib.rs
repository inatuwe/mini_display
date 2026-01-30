pub mod image;
pub mod port;
pub mod protocol;

pub use image::{create_text_image, image_to_rgb565_bytes, DISPLAY_HEIGHT, DISPLAY_WIDTH};
pub use port::{find_display_port, is_display_connected, open_connection, PortInfo};
pub use protocol::send_image_to_display;
