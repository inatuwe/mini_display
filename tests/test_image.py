"""
Tests for image creation functionality.
"""
import pytest
from PIL import Image
from src.image import (
    create_blank_image,
    draw_text,
    create_hello_world_image,
    DISPLAY_WIDTH,
    DISPLAY_HEIGHT,
)


class TestCreateBlankImage:
    """Tests for create_blank_image function."""

    def test_creates_image_with_specified_dimensions(self):
        """Function should create image with specified dimensions."""
        img = create_blank_image(100, 50)
        assert img.size == (100, 50)

    def test_creates_image_with_specified_background_color(self):
        """Function should create image with specified background color."""
        img = create_blank_image(10, 10, bg_color=(255, 0, 0))
        # Check that pixel at (0, 0) is red
        pixel = img.getpixel((0, 0))
        assert pixel == (255, 0, 0)

    def test_default_size_matches_display(self):
        """Default size should match display dimensions (160x80)."""
        img = create_blank_image()
        assert img.size == (DISPLAY_WIDTH, DISPLAY_HEIGHT)

    def test_default_background_is_black(self):
        """Default background should be black."""
        img = create_blank_image(10, 10)
        pixel = img.getpixel((0, 0))
        assert pixel == (0, 0, 0)

    def test_returns_rgb_image(self):
        """Function should return an RGB image."""
        img = create_blank_image()
        assert img.mode == "RGB"


class TestDrawText:
    """Tests for draw_text function."""

    def test_adds_text_to_image(self):
        """Function should add text to image."""
        img = create_blank_image(100, 50, bg_color=(0, 0, 0))
        result = draw_text(img, "Test")
        # Result should be an image
        assert isinstance(result, Image.Image)
        # Image should be modified (not all black anymore)
        # Check that at least some pixels are different
        has_non_black = False
        for x in range(result.size[0]):
            for y in range(result.size[1]):
                if result.getpixel((x, y)) != (0, 0, 0):
                    has_non_black = True
                    break
            if has_non_black:
                break
        assert has_non_black, "Text was not drawn on image"

    def test_accepts_text_position_parameters(self):
        """Function should accept text position parameters."""
        img = create_blank_image(100, 50)
        # Should not raise exception
        result = draw_text(img, "Test", position=(10, 20))
        assert isinstance(result, Image.Image)

    def test_accepts_font_size_parameter(self):
        """Function should accept font size parameter."""
        img = create_blank_image(100, 50)
        # Should not raise exception
        result = draw_text(img, "Test", font_size=20)
        assert isinstance(result, Image.Image)

    def test_accepts_text_color_parameter(self):
        """Function should accept text color parameter."""
        img = create_blank_image(100, 50, bg_color=(0, 0, 0))
        result = draw_text(img, "Test", color=(255, 255, 255))
        # Check that white pixels exist
        has_white = False
        for x in range(result.size[0]):
            for y in range(result.size[1]):
                pixel = result.getpixel((x, y))
                if pixel[0] > 200 and pixel[1] > 200 and pixel[2] > 200:
                    has_white = True
                    break
            if has_white:
                break
        assert has_white, "White text was not drawn"


class TestCreateHelloWorldImage:
    """Tests for create_hello_world_image function."""

    def test_creates_complete_image_with_hello_world_text(self):
        """Function should create complete image with 'Hello World!' text."""
        img = create_hello_world_image()
        assert isinstance(img, Image.Image)
        # Image should not be all black (has text)
        has_non_black = False
        for x in range(img.size[0]):
            for y in range(img.size[1]):
                if img.getpixel((x, y)) != (0, 0, 0):
                    has_non_black = True
                    break
            if has_non_black:
                break
        assert has_non_black, "Hello World text was not drawn"

    def test_image_has_correct_dimensions(self):
        """Image should have correct dimensions for display."""
        img = create_hello_world_image()
        assert img.size == (DISPLAY_WIDTH, DISPLAY_HEIGHT)

    def test_returns_rgb_image(self):
        """Function should return an RGB image."""
        img = create_hello_world_image()
        assert img.mode == "RGB"
