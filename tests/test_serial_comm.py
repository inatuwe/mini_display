"""
Tests for serial communication functionality.
"""
import pytest
from unittest.mock import MagicMock, patch
import serial
from src.serial_comm import (
    send_bytes,
    create_display_command,
)


class TestSendBytes:
    """Tests for send_bytes function."""

    def test_sends_bytes_to_serial_connection(self):
        """Function should send bytes to serial connection."""
        mock_connection = MagicMock()
        mock_connection.write.return_value = 5
        
        data = b'\x01\x02\x03\x04\x05'
        result = send_bytes(mock_connection, data)
        
        mock_connection.write.assert_called_once_with(data)
        assert result is True

    def test_handles_connection_errors(self):
        """Function should handle connection errors."""
        mock_connection = MagicMock()
        mock_connection.write.side_effect = serial.SerialException("Write failed")
        
        data = b'\x01\x02\x03'
        result = send_bytes(mock_connection, data)
        
        assert result is False

    def test_returns_success_status(self):
        """Function should return success/failure status."""
        mock_connection = MagicMock()
        mock_connection.write.return_value = 10
        
        data = b'\x00' * 10
        result = send_bytes(mock_connection, data)
        
        assert isinstance(result, bool)
        assert result is True

    def test_handles_partial_write(self):
        """Function should handle partial write scenarios."""
        mock_connection = MagicMock()
        # Simulate partial write (wrote fewer bytes than requested)
        mock_connection.write.return_value = 3
        
        data = b'\x01\x02\x03\x04\x05'
        # Should still return True as write was called successfully
        result = send_bytes(mock_connection, data)
        
        assert result is True

    def test_handles_none_connection(self):
        """Function should handle None connection gracefully."""
        result = send_bytes(None, b'\x01\x02\x03')
        assert result is False


class TestCreateDisplayCommand:
    """Tests for create_display_command function."""

    def test_wraps_data_with_header(self):
        """Function should wrap data with correct header."""
        image_data = b'\x00\x01\x02'
        result = create_display_command(image_data)
        
        # Result should be bytes
        assert isinstance(result, bytes)
        # Result should be larger than original data (has header/footer)
        assert len(result) >= len(image_data)

    def test_includes_image_data(self):
        """Function should include the image data."""
        image_data = b'\xAA\xBB\xCC\xDD'
        result = create_display_command(image_data)
        
        # Image data should be present in result
        assert image_data in result

    def test_handles_empty_data(self):
        """Function should handle empty data."""
        result = create_display_command(b'')
        
        assert isinstance(result, bytes)
