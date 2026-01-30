"""
Tests for error handling across the display modules.
"""
import pytest
from unittest.mock import patch, MagicMock
import serial


class TestGracefulDisconnection:
    """Tests for graceful handling when display disconnects during operation."""

    def test_send_bytes_handles_disconnected_display(self):
        """send_bytes should handle disconnection gracefully."""
        from src.serial_comm import send_bytes
        
        mock_conn = MagicMock()
        mock_conn.write.side_effect = serial.SerialException("Device disconnected")
        
        result = send_bytes(mock_conn, b"test data")
        
        assert result is False

    def test_open_connection_handles_device_not_found(self):
        """open_connection should raise appropriate error for missing device."""
        from src.com_ports import open_connection
        
        mock_port = MagicMock()
        mock_port.device = "COM99"  # Non-existent port
        
        with patch('src.com_ports.serial.Serial') as mock_serial:
            mock_serial.side_effect = serial.SerialException("could not open port")
            
            with pytest.raises(serial.SerialException):
                open_connection(mock_port)


class TestPermissionErrors:
    """Tests for graceful handling of permission errors."""

    def test_open_connection_handles_permission_denied(self):
        """open_connection should raise appropriate error for permission issues."""
        from src.com_ports import open_connection
        
        mock_port = MagicMock()
        mock_port.device = "COM3"
        
        with patch('src.com_ports.serial.Serial') as mock_serial:
            mock_serial.side_effect = serial.SerialException("Access denied")
            
            with pytest.raises(serial.SerialException):
                open_connection(mock_port)

    def test_hello_world_handles_permission_error(self):
        """hello_world main should handle permission errors gracefully."""
        mock_port = MagicMock()
        mock_port.device = "COM3"
        
        with patch('hello_world.find_display_port') as mock_find:
            with patch('hello_world.open_connection') as mock_open:
                mock_find.return_value = mock_port
                mock_open.side_effect = serial.SerialException("Access denied")
                
                from hello_world import main
                result = main()
                
                # Should return False and not crash
                assert result is False


class TestHelpfulErrorMessages:
    """Tests for helpful error messages to users."""

    def test_hello_world_shows_helpful_message_when_not_found(self):
        """hello_world should show helpful message when display not found."""
        import io
        
        with patch('hello_world.find_display_port') as mock_find:
            mock_find.return_value = None
            
            from hello_world import main
            
            captured = io.StringIO()
            with patch('sys.stdout', captured):
                main()
            
            output = captured.getvalue()
            # Should mention USB-C or driver
            assert "usb" in output.lower() or "driver" in output.lower() or "ch340" in output.lower()

    def test_hello_world_shows_error_details_on_connection_failure(self):
        """hello_world should show error details when connection fails."""
        import io
        
        mock_port = MagicMock()
        mock_port.device = "COM3"
        
        with patch('hello_world.find_display_port') as mock_find:
            with patch('hello_world.open_connection') as mock_open:
                mock_find.return_value = mock_port
                mock_open.side_effect = Exception("Port busy")
                
                from hello_world import main
                
                captured = io.StringIO()
                with patch('sys.stdout', captured):
                    result = main()
                
                output = captured.getvalue()
                # Should show error message
                assert result is False
                assert "error" in output.lower() or "port busy" in output.lower()

    def test_detect_display_shows_instructions_when_not_found(self):
        """detect_display should show helpful instructions when not found."""
        import io
        
        with patch('detect_display.is_display_fs_connected') as mock_connected:
            mock_connected.return_value = False
            
            from detect_display import main
            
            captured = io.StringIO()
            with patch('sys.stdout', captured):
                main()
            
            output = captured.getvalue()
            # Should be helpful
            assert "not" in output.lower() and ("found" in output.lower() or "connected" in output.lower())


class TestCloseConnectionRobustness:
    """Tests for robust connection closing."""

    def test_close_connection_handles_already_closed(self):
        """close_connection should handle already closed connections."""
        from src.com_ports import close_connection
        
        mock_conn = MagicMock()
        mock_conn.close.side_effect = serial.SerialException("Port not open")
        
        # Should not raise exception
        close_connection(mock_conn)

    def test_close_connection_handles_none(self):
        """close_connection should handle None connection."""
        from src.com_ports import close_connection
        
        # Should not raise exception
        close_connection(None)
