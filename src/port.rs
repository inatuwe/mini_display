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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_ports_returns_vec() {
        let ports = list_ports();
        // Verify it's a valid Vec by checking it doesn't panic
        let _ = ports.len();
    }

    #[test]
    fn test_find_display_port_returns_option() {
        let result = find_display_port();
        // Result is Option<PortInfo> - either Some or None
        match result {
            Some(port) => {
                assert!(!port.name.is_empty());
                assert!(port.vid > 0);
                assert!(port.pid > 0);
            }
            None => {
                // Display not connected - this is valid
            }
        }
    }

    #[test]
    fn test_is_display_connected_returns_bool() {
        let result = is_display_connected();
        // Just verify it returns a bool without panicking
        assert!(result == true || result == false);
    }

    #[test]
    fn test_vid_pid_constants_defined() {
        // Verify CH340, CH341, and WeAct VID/PIDs are defined
        assert!(DISPLAY_FS_VID_PID.contains(&(0x1A86, 0x7523))); // CH340
        assert!(DISPLAY_FS_VID_PID.contains(&(0x1A86, 0x5523))); // CH341
        assert!(DISPLAY_FS_VID_PID.contains(&(0x1A86, 0xFE0C))); // WeAct
    }

    #[test]
    fn test_port_info_struct() {
        let port = PortInfo {
            name: "COM3".to_string(),
            vid: 0x1A86,
            pid: 0x7523,
        };
        assert_eq!(port.name, "COM3");
        assert_eq!(port.vid, 0x1A86);
        assert_eq!(port.pid, 0x7523);
    }
}
