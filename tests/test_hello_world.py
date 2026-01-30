"""
Tests for hello world script.
"""
import pytest
from unittest.mock import patch, MagicMock
import io


class TestHelloWorldScript:
    """Tests for hello_world.py script."""

    def test_script_runs_without_error_when_connected(self):
        """Script should run without error when display connected."""
        # Setup mock for display found
        mock_port = MagicMock()
        mock_port.device = "COM3"
        mock_port.description = "USB-SERIAL CH340"
        
        # Patch where the functions are USED (in hello_world module), not where defined
        with patch('hello_world.find_display_port') as mock_find_port:
            with patch('hello_world.open_connection') as mock_open:
                with patch('hello_world.close_connection') as mock_close:
                    with patch('hello_world.send_image_to_display') as mock_send:
                        mock_find_port.return_value = mock_port
                        mock_conn = MagicMock()
                        mock_open.return_value = mock_conn
                        mock_send.return_value = True
                        
                        from hello_world import main
                        
                        result = main()
                        
                        assert result is True

    def test_script_shows_error_when_not_connected(self):
        """Script should show error message when display not connected."""
        with patch('hello_world.find_display_port') as mock_find_port:
            mock_find_port.return_value = None
            
            from hello_world import main
            
            # Capture stdout
            captured_output = io.StringIO()
            with patch('sys.stdout', captured_output):
                result = main()
            
            output = captured_output.getvalue()
            assert result is False
            assert "not found" in output.lower() or "not connected" in output.lower() or "error" in output.lower()

    def test_script_creates_and_sends_image(self):
        """Script should create and send hello world image."""
        mock_port = MagicMock()
        mock_port.device = "COM3"
        
        with patch('hello_world.find_display_port') as mock_find_port:
            with patch('hello_world.open_connection') as mock_open:
                with patch('hello_world.close_connection') as mock_close:
                    with patch('hello_world.send_image_to_display') as mock_send:
                        mock_find_port.return_value = mock_port
                        mock_conn = MagicMock()
                        mock_open.return_value = mock_conn
                        mock_send.return_value = True
                        
                        from hello_world import main
                        
                        main()
                        
                        # Verify connection was opened and closed
                        mock_open.assert_called_once()
                        mock_close.assert_called_once()
                        # Verify data was sent
                        mock_send.assert_called()
