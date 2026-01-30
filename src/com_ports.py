"""
COM port enumeration functionality.
"""
from serial.tools import list_ports


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
