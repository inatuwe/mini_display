"""
COM port enumeration functionality.
"""
from serial.tools import list_ports


# Known VID/PID combinations for Display FS V1
# CH340/CH341 USB-Serial chip commonly used in WeAct Studio displays
DISPLAY_FS_VID_PID = [
    (0x1A86, 0x7523),  # CH340
    (0x1A86, 0x5523),  # CH341
]


def list_com_ports():
    """
    List all available COM ports on the system.
    
    Returns:
        list: List of port objects with device, description, and hwid attributes.
    """
    return list(list_ports.comports())


def format_port_info(port):
    """
    Format port information as a readable string.
    
    Args:
        port: Port object with device, description, and hwid attributes.
        
    Returns:
        str: Formatted string containing port name and description.
    """
    return f"{port.device}: {port.description} [{port.hwid}]"


def is_display_fs_connected(ports=None):
    """
    Check if Display FS V1 is connected by matching known VID/PID.
    
    Args:
        ports: Optional list of port objects. If None, scans system ports.
        
    Returns:
        bool: True if Display FS V1 is found, False otherwise.
    """
    if ports is None:
        ports = list_com_ports()
    
    for port in ports:
        vid = getattr(port, 'vid', None)
        pid = getattr(port, 'pid', None)
        if vid is not None and pid is not None:
            if (vid, pid) in DISPLAY_FS_VID_PID:
                return True
    return False


def find_display_port(ports=None):
    """
    Find and return the port object for Display FS V1.
    
    Args:
        ports: Optional list of port objects. If None, scans system ports.
        
    Returns:
        Port object if found, None otherwise.
    """
    if ports is None:
        ports = list_com_ports()
    
    for port in ports:
        vid = getattr(port, 'vid', None)
        pid = getattr(port, 'pid', None)
        if vid is not None and pid is not None:
            if (vid, pid) in DISPLAY_FS_VID_PID:
                return port
    return None
