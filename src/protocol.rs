use crate::image::Orientation;
use serialport::SerialPort;
use std::io::Write;
use std::thread::sleep;
use std::time::Duration;
use thiserror::Error;

const CMD_SET_ORIENTATION: u8 = 0x02;
const CMD_SET_BITMAP: u8 = 0x05;
const CMD_END: u8 = 0x0A;

#[derive(Error, Debug)]
pub enum ProtocolError {
    #[error("Failed to send data: {0}")]
    SendFailed(#[from] std::io::Error),
}

pub fn create_bitmap_header() -> [u8; 10] {
    create_bitmap_header_oriented(Orientation::default())
}

/// Physical display dimensions (always 80x160 portrait)
const PHYSICAL_WIDTH: u16 = 80;
const PHYSICAL_HEIGHT: u16 = 160;

pub fn create_bitmap_header_oriented(_orientation: Orientation) -> [u8; 10] {
    // Always use physical dimensions - rotation is handled in image data
    let x0: u16 = 0;
    let y0: u16 = 0;
    let x1: u16 = PHYSICAL_WIDTH - 1;
    let y1: u16 = PHYSICAL_HEIGHT - 1;

    [
        CMD_SET_BITMAP,
        (x0 & 0xFF) as u8,
        (x0 >> 8) as u8,
        (y0 & 0xFF) as u8,
        (y0 >> 8) as u8,
        (x1 & 0xFF) as u8,
        (x1 >> 8) as u8,
        (y1 & 0xFF) as u8,
        (y1 >> 8) as u8,
        CMD_END,
    ]
}

pub fn send_image_to_display(
    port: &mut Box<dyn SerialPort>,
    image_data: &[u8],
) -> Result<(), ProtocolError> {
    send_image_to_display_oriented(port, image_data, Orientation::default())
}

/// Create orientation command to initialize display orientation
fn create_orientation_command(orientation: Orientation) -> [u8; 3] {
    // Orientation values: 0=portrait, 1=landscape, 2=portrait_flip, 3=landscape_flip
    let orientation_value = match orientation {
        Orientation::Portrait => 0,
        Orientation::Landscape => 1,
    };
    [CMD_SET_ORIENTATION, orientation_value, CMD_END]
}

pub fn send_image_to_display_oriented(
    port: &mut Box<dyn SerialPort>,
    image_data: &[u8],
    orientation: Orientation,
) -> Result<(), ProtocolError> {
    port.clear(serialport::ClearBuffer::All)
        .map_err(|e| ProtocolError::SendFailed(std::io::Error::other(e)))?;

    // Send orientation command first
    let orient_cmd = create_orientation_command(orientation);
    port.write_all(&orient_cmd)?;
    port.flush()?;
    sleep(Duration::from_millis(50));

    let header = create_bitmap_header_oriented(orientation);
    port.write_all(&header)?;
    port.flush()?;

    let chunk_size = PHYSICAL_WIDTH as usize * 4;
    for chunk in image_data.chunks(chunk_size) {
        port.write_all(chunk)?;
    }

    port.flush()?;
    sleep(Duration::from_millis(100));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bitmap_header_structure() {
        let header = create_bitmap_header();
        // Header should be 10 bytes
        assert_eq!(header.len(), 10);
        // First byte is CMD_SET_BITMAP (0x05)
        assert_eq!(header[0], CMD_SET_BITMAP);
        // Last byte is CMD_END (0x0A)
        assert_eq!(header[9], CMD_END);
    }

    #[test]
    fn test_bitmap_header_always_physical_dimensions() {
        // Both orientations use physical 80x160 dimensions
        for orientation in [Orientation::Landscape, Orientation::Portrait] {
            let header = create_bitmap_header_oriented(orientation);
            // x0 = 0, y0 = 0
            assert_eq!(header[1], 0x00); // x0 low
            assert_eq!(header[2], 0x00); // x0 high
            assert_eq!(header[3], 0x00); // y0 low
            assert_eq!(header[4], 0x00); // y0 high
                                         // Physical: x1 = 79, y1 = 159
            assert_eq!(header[5], 0x4F); // x1 low (79 = 0x4F)
            assert_eq!(header[6], 0x00); // x1 high
            assert_eq!(header[7], 0x9F); // y1 low (159 = 0x9F)
            assert_eq!(header[8], 0x00); // y1 high
        }
    }

    #[test]
    fn test_command_constants() {
        assert_eq!(CMD_SET_BITMAP, 0x05);
        assert_eq!(CMD_END, 0x0A);
    }

    #[test]
    fn test_chunk_size_physical() {
        // Always uses physical width: 80 * 4 = 320
        assert_eq!(PHYSICAL_WIDTH as usize * 4, 320);
    }
}
