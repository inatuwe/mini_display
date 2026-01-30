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
        .map_err(|e| ProtocolError::SendFailed(std::io::Error::new(std::io::ErrorKind::Other, e)))?;

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
