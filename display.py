#!/usr/bin/env python3
"""
Hello World Display Script

Displays "Hello World!" on the WeAct Studio Display FS V1 (0.96 inch).
"""
import argparse
from src.com_ports import (
    find_display_port,
    open_connection,
    close_connection,
)
from src.image import (
    create_text_image,
    image_to_bytes,
)
from src.serial_comm import (
    send_image_to_display,
)


def main(text="Hello World!", font_size=14):
    """
    Main function to display text on the display.
    
    Args:
        text: Text to display (default: "Hello World!").
        font_size: Font size in pixels (default: 14).
    
    Returns:
        bool: True if successful, False otherwise.
    """
    print("=" * 50)
    print("Hello World Display Script")
    print("=" * 50)
    print()
    
    # Find display
    print("Looking for Display FS V1...")
    port = find_display_port()
    
    if port is None:
        print("✗ ERROR: Display FS V1 not found!")
        print("  Make sure the display is connected via USB-C")
        print("  and the CH340/CH341 driver is installed.")
        return False
    
    print(f"✓ Found display on {port.device}")
    
    # Create image
    print(f"Creating image with text: '{text}' (font size: {font_size})")
    image = create_text_image(text, font_size)
    image_data = image_to_bytes(image)
    print(f"  Image size: {len(image_data)} bytes")
    
    # Open connection
    print(f"Opening connection to {port.device}...")
    connection = None
    try:
        connection = open_connection(port)
        print("✓ Connection opened")
        
        # Send image
        print("Sending image to display...")
        success = send_image_to_display(connection, image_data)
        
        if success:
            print("✓ Image sent successfully!")
            print()
            print("Hello World should now be displayed!")
            return True
        else:
            print("✗ Failed to send image")
            return False
            
    except Exception as e:
        print(f"✗ Error: {e}")
        return False
    finally:
        if connection:
            close_connection(connection)
            print("Connection closed")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Display text on Display FS V1")
    parser.add_argument("-s", "--font-size", type=int, default=14,
                        help="Font size in pixels (default: 14)")
    parser.add_argument("-t", "--text", default="Hello World!",
                        help="Text to display (default: 'Hello World!')")
    args = parser.parse_args()
    
    result = main(text=args.text, font_size=args.font_size)
    exit(0 if result else 1)
