"""
Image creation functionality for Display FS V1.
"""
from PIL import Image, ImageDraw, ImageFont


# Display dimensions for WeAct Studio Display FS V1 (0.96 inch)
# Landscape mode: 160 wide x 80 tall
DISPLAY_WIDTH = 160
DISPLAY_HEIGHT = 80


def create_blank_image(width=DISPLAY_WIDTH, height=DISPLAY_HEIGHT, bg_color=(0, 0, 0)):
    """
    Create a blank RGB image with specified dimensions and background color.
    
    Args:
        width: Image width in pixels (default: DISPLAY_WIDTH).
        height: Image height in pixels (default: DISPLAY_HEIGHT).
        bg_color: Background color as RGB tuple (default: black).
        
    Returns:
        PIL Image object.
    """
    return Image.new("RGB", (width, height), bg_color)


def draw_text(image, text, position=None, font_size=12, color=(255, 255, 255)):
    """
    Draw text on an image.
    
    Args:
        image: PIL Image object to draw on.
        text: Text string to draw.
        position: Tuple (x, y) for text position. If None, centers text.
        font_size: Font size in pixels (default: 12).
        color: Text color as RGB tuple (default: white).
        
    Returns:
        PIL Image object with text drawn.
    """
    draw = ImageDraw.Draw(image)
    
    # Try to use a default font, fall back to basic font if not available
    try:
        font = ImageFont.truetype("arial.ttf", font_size)
    except (IOError, OSError):
        # Fall back to default font
        font = ImageFont.load_default()
    
    # Get text bounding box for centering
    bbox = draw.textbbox((0, 0), text, font=font)
    text_width = bbox[2] - bbox[0]
    text_height = bbox[3] - bbox[1]
    
    # Calculate position (center if not specified)
    if position is None:
        x = (image.width - text_width) // 2
        y = (image.height - text_height) // 2
    else:
        x, y = position
    
    draw.text((x, y), text, fill=color, font=font)
    
    return image


def create_hello_world_image():
    """
    Create an image with "Hello World!" text for the display.
    
    Returns:
        PIL Image object with "Hello World!" text centered.
    """
    image = create_blank_image()
    # Landscape mode - single line fits better
    return draw_text(image, "Hello from SoCraTes!", font_size=14)


def image_to_bytes(image):
    """
    Convert a PIL image to RGB565 bytes for serial transmission.
    
    RGB565 format uses 2 bytes per pixel:
    - 5 bits for Red (bits 15-11)
    - 6 bits for Green (bits 10-5)
    - 5 bits for Blue (bits 4-0)
    
    WeAct Studio display uses little-endian byte order.
    
    Args:
        image: PIL Image object in RGB mode.
        
    Returns:
        bytes: Image data in RGB565 format (little-endian).
    """
    # Ensure image is in RGB mode
    if image.mode != "RGB":
        image = image.convert("RGB")
    
    pixels = image.load()
    width, height = image.size
    data = bytearray()
    
    for y in range(height):
        for x in range(width):
            r, g, b = pixels[x, y]
            # Convert to RGB565
            # R: 8 bits -> 5 bits (shift right by 3)
            # G: 8 bits -> 6 bits (shift right by 2)
            # B: 8 bits -> 5 bits (shift right by 3)
            r5 = (r >> 3) & 0x1F
            g6 = (g >> 2) & 0x3F
            b5 = (b >> 3) & 0x1F
            # Pack into 16-bit value: RRRRR GGGGGG BBBBB
            rgb565 = (r5 << 11) | (g6 << 5) | b5
            # Little-endian: low byte first
            data.append(rgb565 & 0xFF)
            data.append((rgb565 >> 8) & 0xFF)
    
    return bytes(data)
