use crate::image::{DISPLAY_HEIGHT, DISPLAY_WIDTH};
use serialport::SerialPort;
use std::io::Write;
use std::thread::sleep;
use std::time::Duration;
use thiserror::Error;

const CMD_SET_BITMAP: u8 = 0x05;
const CMD_END: u8 = 0x0A;
const CHUNK_SIZE: usize = DISPLAY_WIDTH as usize * 4;

#[derive(Error, Debug)]
pub enum ProtocolError {
    #[error("Failed to send data: {0}")]
    SendFailed(#[from] std::io::Error),
}

pub fn create_bitmap_header() -> [u8; 10] {
    let x0: u16 = 0;
    let y0: u16 = 0;
    let x1: u16 = DISPLAY_WIDTH as u16 - 1;
    let y1: u16 = DISPLAY_HEIGHT as u16 - 1;

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
    port.clear(serialport::ClearBuffer::All)
        .map_err(|e| ProtocolError::SendFailed(std::io::Error::other(e)))?;

    let header = create_bitmap_header();
    port.write_all(&header)?;
    port.flush()?;

    for chunk in image_data.chunks(CHUNK_SIZE) {
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
    fn test_bitmap_header_coordinates() {
        let header = create_bitmap_header();
        // x0 = 0 (little-endian: bytes 1-2)
        assert_eq!(header[1], 0x00); // x0 low
        assert_eq!(header[2], 0x00); // x0 high
                                     // y0 = 0 (little-endian: bytes 3-4)
        assert_eq!(header[3], 0x00); // y0 low
        assert_eq!(header[4], 0x00); // y0 high
                                     // x1 = 159 (little-endian: bytes 5-6)
        assert_eq!(header[5], 0x9F); // x1 low (159 = 0x9F)
        assert_eq!(header[6], 0x00); // x1 high
                                     // y1 = 79 (little-endian: bytes 7-8)
        assert_eq!(header[7], 0x4F); // y1 low (79 = 0x4F)
        assert_eq!(header[8], 0x00); // y1 high
    }

    #[test]
    fn test_command_constants() {
        assert_eq!(CMD_SET_BITMAP, 0x05);
        assert_eq!(CMD_END, 0x0A);
    }

    #[test]
    fn test_chunk_size() {
        // CHUNK_SIZE = DISPLAY_WIDTH * 4 = 160 * 4 = 640
        assert_eq!(CHUNK_SIZE, 640);
    }
}
