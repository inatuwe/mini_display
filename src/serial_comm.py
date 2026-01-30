"""
Serial communication functionality for Display FS V1.
"""
import serial


# Display command constants
# These may need adjustment based on actual display protocol
COMMAND_HEADER = b'\x00'  # Placeholder header byte
COMMAND_DISPLAY = 0x01    # Display image command


def send_bytes(connection, data):
    """
    Send bytes to serial connection.
    
    Args:
        connection: Serial connection object.
        data: Bytes to send.
        
    Returns:
        bool: True if send was successful, False otherwise.
    """
    if connection is None:
        return False
    
    try:
        connection.write(data)
        return True
    except serial.SerialException:
        return False
    except Exception:
        return False


def create_display_command(image_data):
    """
    Wrap image data with display command header/footer.
    
    The exact protocol depends on the display firmware.
    This is a basic implementation that can be adjusted
    based on the actual display protocol.
    
    Args:
        image_data: Image data bytes (RGB565 format).
        
    Returns:
        bytes: Complete command packet with header and image data.
    """
    # Basic protocol: header + command + length + data
    # This may need adjustment based on actual display protocol
    length = len(image_data)
    length_bytes = length.to_bytes(4, byteorder='big')
    
    # Build command packet
    command = bytearray()
    command.append(COMMAND_DISPLAY)  # Command byte
    command.extend(length_bytes)      # Data length (4 bytes)
    command.extend(image_data)        # Image data
    
    return bytes(command)
