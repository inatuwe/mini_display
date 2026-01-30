#!/usr/bin/env python3
"""
Display FS V1 Detection Script

Detects if the WeAct Studio Display FS V1 (0.96 inch) is connected
and reports its status.
"""
from src.com_ports import (
    list_com_ports,
    format_port_info,
    is_display_fs_connected,
    find_display_port,
)


def main():
    """
    Main function to detect and report Display FS V1 status.
    
    Returns:
        bool: True if display is found, False otherwise.
    """
    print("=" * 50)
    print("Display FS V1 Detection")
    print("=" * 50)
    print()
    
    # List all COM ports
    ports = list_com_ports()
    print(f"Found {len(ports)} COM port(s):")
    for port in ports:
        print(f"  - {format_port_info(port)}")
    print()
    
    # Check for Display FS V1
    display_port = find_display_port(ports)
    
    if display_port:
        print("✓ Display FS V1 FOUND!")
        print(f"  Port: {display_port.device}")
        print(f"  Description: {display_port.description}")
        print(f"  HWID: {display_port.hwid}")
        return True
    else:
        print("✗ Display FS V1 NOT FOUND")
        print("  Make sure the display is connected via USB-C")
        print("  and the CH340/CH341 driver is installed.")
        return False


if __name__ == "__main__":
    result = main()
    exit(0 if result else 1)
