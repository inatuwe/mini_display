use serialport::{SerialPort, SerialPortInfo, SerialPortType};
use std::time::Duration;
use thiserror::Error;

const BAUD_RATE: u32 = 115200;
const TIMEOUT_MS: u64 = 1000;

const DISPLAY_FS_VID_PID: [(u16, u16); 3] = [
    (0x1A86, 0x7523), // CH340
    (0x1A86, 0x5523), // CH341
    (0x1A86, 0xFE0C), // WeAct Studio Display FS V1
];

#[derive(Error, Debug)]
pub enum PortError {
    #[error("Display not found")]
    NotFound,
    #[error("Failed to open port: {0}")]
    OpenFailed(#[from] serialport::Error),
}

#[derive(Debug, Clone)]
pub struct PortInfo {
    pub name: String,
    pub vid: u16,
    pub pid: u16,
}

pub fn list_ports() -> Vec<SerialPortInfo> {
    serialport::available_ports().unwrap_or_default()
}

pub fn find_display_port() -> Option<PortInfo> {
    for port in list_ports() {
        if let SerialPortType::UsbPort(usb_info) = &port.port_type {
            let vid = usb_info.vid;
            let pid = usb_info.pid;
            if DISPLAY_FS_VID_PID.contains(&(vid, pid)) {
                return Some(PortInfo {
                    name: port.port_name,
                    vid,
                    pid,
                });
            }
        }
    }
    None
}

pub fn is_display_connected() -> bool {
    find_display_port().is_some()
}

pub fn open_connection(port: &PortInfo) -> Result<Box<dyn SerialPort>, PortError> {
    let connection = serialport::new(&port.name, BAUD_RATE)
        .timeout(Duration::from_millis(TIMEOUT_MS))
        .open()?;
    Ok(connection)
}
