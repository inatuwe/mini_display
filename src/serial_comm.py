"""
Serial communication functionality for Display FS V1.

Protocol based on WeAct Studio's implementation:
https://github.com/WeActStudio/WeActStudio.SystemMonitor
"""
import serial
import time


# Display command constants (WeAct Studio Display FS V1 0.96")
CMD_SET_BITMAP = 0x05      # Set bitmap command
CMD_END = 0x0A             # End of command marker

# Display dimensions for 0.96" model (landscape)
DISPLAY_WIDTH = 160
DISPLAY_HEIGHT = 80

# Chunk size for sending data (display width * 4 bytes as per WeAct protocol)
CHUNK_SIZE = DISPLAY_WIDTH * 4


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


def chunked(data, chunk_size):
    """
    Split data into chunks of specified size.
    
    Args:
        data: Bytes or bytearray to split.
        chunk_size: Size of each chunk.
        
    Yields:
        Chunks of data.
    """
    for i in range(0, len(data), chunk_size):
        yield data[i:i + chunk_size]


def send_image_to_display(connection, image_data):
    """
    Send image data to the display using the WeAct protocol.
    
    Sends the header first, then sends image data in chunks.
    
    Args:
        connection: Serial connection object.
        image_data: Image data bytes (RGB565 little-endian format).
        
    Returns:
        bool: True if successful, False otherwise.
    """
    if connection is None:
        return False
    
    try:
        # Flush any pending data
        connection.reset_input_buffer()
        connection.reset_output_buffer()
        
        # Create and send header
        header = create_bitmap_header()
        connection.write(header)
        connection.flush()
        
        # Send image data in chunks
        for chunk in chunked(image_data, CHUNK_SIZE):
            connection.write(chunk)
        
        # Ensure all data is sent
        connection.flush()
        
        # Small delay to ensure display processes the data
        time.sleep(0.1)
        return True
        
    except serial.SerialException:
        return False
    except Exception:
        return False


def create_bitmap_header():
    """
    Create the bitmap command header for WeAct Studio Display FS V1 0.96".
    
    Returns:
        bytes: 10-byte command header.
    """
    # Full screen coordinates (0,0) to (width-1, height-1)
    x0, y0 = 0, 0
    x1, y1 = DISPLAY_WIDTH - 1, DISPLAY_HEIGHT - 1
    
    # Build command header (10 bytes)
    header = bytearray(10)
    header[0] = CMD_SET_BITMAP
    header[1] = x0 & 0xFF
    header[2] = (x0 >> 8) & 0xFF
    header[3] = y0 & 0xFF
    header[4] = (y0 >> 8) & 0xFF
    header[5] = x1 & 0xFF
    header[6] = (x1 >> 8) & 0xFF
    header[7] = y1 & 0xFF
    header[8] = (y1 >> 8) & 0xFF
    header[9] = CMD_END
    
    return bytes(header)


def create_display_command(image_data):
    """
    Create the bitmap command header for WeAct Studio Display FS V1 0.96".
    
    Protocol:
    - Byte 0: CMD_SET_BITMAP (0x05)
    - Bytes 1-2: x0 (start X, little-endian)
    - Bytes 3-4: y0 (start Y, little-endian)
    - Bytes 5-6: x1 (end X, little-endian)  
    - Bytes 7-8: y1 (end Y, little-endian)
    - Byte 9: CMD_END (0x0A)
    - Followed by: RGB565 little-endian image data
    
    Args:
        image_data: Image data bytes (RGB565 little-endian format).
        
    Returns:
        bytes: Complete command packet with header and image data.
    """
    return create_bitmap_header() + image_data
