"""
Tests for main detection script.
"""
import pytest
from unittest.mock import patch, MagicMock
import sys
import io


class TestDetectDisplayScript:
    """Tests for detect_display.py script."""

    @patch('src.com_ports.list_com_ports')
    def test_script_runs_without_error(self, mock_list_ports):
        """Script should run without error."""
        mock_list_ports.return_value = []
        
        # Import and run the main function
        from detect_display import main
        
        # Should not raise exception
        result = main()
        assert result is not None

    @patch('src.com_ports.find_display_port')
    @patch('src.com_ports.list_com_ports')
    def test_script_outputs_connection_status_when_found(self, mock_list_ports, mock_find_port):
        """Script should output connection status message when display found."""
        # Setup mock for display found
        mock_port = MagicMock()
        mock_port.device = "COM3"
        mock_port.description = "USB-SERIAL CH340"
        mock_find_port.return_value = mock_port
        mock_list_ports.return_value = [mock_port]
        
        from detect_display import main
        
        # Capture stdout
        captured_output = io.StringIO()
        with patch('sys.stdout', captured_output):
            main()
        
        output = captured_output.getvalue()
        assert "COM3" in output or "found" in output.lower() or "connected" in output.lower()

    @patch('src.com_ports.find_display_port')
    @patch('src.com_ports.list_com_ports')
    def test_script_outputs_not_found_when_missing(self, mock_list_ports, mock_find_port):
        """Script should output not found message when display missing."""
        mock_find_port.return_value = None
        mock_list_ports.return_value = []
        
        from detect_display import main
        
        # Capture stdout
        captured_output = io.StringIO()
        with patch('sys.stdout', captured_output):
            main()
        
        output = captured_output.getvalue()
        assert "not found" in output.lower() or "not connected" in output.lower()
