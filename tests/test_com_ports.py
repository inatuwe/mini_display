"""
Tests for COM port enumeration functionality.
"""
import pytest
from unittest.mock import Mock, patch, MagicMock
import serial
from src.com_ports import (
    list_com_ports,
    format_port_info,
    is_display_fs_connected,
    find_display_port,
    open_connection,
    close_connection,
    DISPLAY_FS_VID_PID,
    DEFAULT_BAUD_RATE,
    DEFAULT_TIMEOUT,
)


class TestListComPorts:
    """Tests for list_com_ports function."""

    def test_returns_list(self):
        """Function should return a list (can be empty)."""
        result = list_com_ports()
        assert isinstance(result, list)

    def test_items_have_required_attributes(self):
        """Each item in list should have device, description, hwid attributes."""
        result = list_com_ports()
        for port in result:
            assert hasattr(port, 'device')
            assert hasattr(port, 'description')
            assert hasattr(port, 'hwid')


class TestFormatPortInfo:
    """Tests for format_port_info function."""

    def test_formats_as_string(self):
        """Function should return a formatted string."""
        ports = list_com_ports()
        if ports:
            result = format_port_info(ports[0])
            assert isinstance(result, str)

    def test_contains_port_name_and_description(self):
        """Output should contain port name and description."""
        ports = list_com_ports()
        if ports:
            port = ports[0]
            result = format_port_info(port)
            assert port.device in result
            assert port.description in result

    def test_formats_mock_port(self):
        """Test with a mock port object."""
        class MockPort:
            device = "COM1"
            description = "Test Device"
            hwid = "USB VID:PID=1234:5678"
        
        result = format_port_info(MockPort())
        assert "COM1" in result
        assert "Test Device" in result


class TestIsDisplayFsConnected:
    """Tests for is_display_fs_connected function."""

    def test_returns_true_if_known_vid_pid_found(self):
        """Function should return True if known VID/PID found."""
        class MockPort:
            device = "COM3"
            description = "USB-SERIAL CH340"
            hwid = "USB VID:PID=1A86:7523"
            vid = 0x1A86
            pid = 0x7523
        
        mock_ports = [MockPort()]
        result = is_display_fs_connected(mock_ports)
        assert result is True

    def test_returns_false_if_no_matching_device(self):
        """Function should return False if no matching device."""
        class MockPort:
            device = "COM1"
            description = "Some Other Device"
            hwid = "USB VID:PID=ABCD:1234"
            vid = 0xABCD
            pid = 0x1234
        
        mock_ports = [MockPort()]
        result = is_display_fs_connected(mock_ports)
        assert result is False

    def test_handles_empty_port_list(self):
        """Function should return False for empty port list."""
        result = is_display_fs_connected([])
        assert result is False

    def test_uses_system_ports_if_none_provided(self):
        """Function should use system ports if no list provided."""
        result = is_display_fs_connected()
        assert isinstance(result, bool)


class TestFindDisplayPort:
    """Tests for find_display_port function."""

    def test_returns_port_if_found(self):
        """Function should return port object if found."""
        class MockPort:
            device = "COM3"
            description = "USB-SERIAL CH340"
            hwid = "USB VID:PID=1A86:7523"
            vid = 0x1A86
            pid = 0x7523
        
        mock_port = MockPort()
        mock_ports = [mock_port]
        result = find_display_port(mock_ports)
        assert result is mock_port

    def test_returns_none_if_not_found(self):
        """Function should return None if not found."""
        class MockPort:
            device = "COM1"
            description = "Some Other Device"
            hwid = "USB VID:PID=ABCD:1234"
            vid = 0xABCD
            pid = 0x1234
        
        mock_ports = [MockPort()]
        result = find_display_port(mock_ports)
        assert result is None

    def test_returns_none_for_empty_list(self):
        """Function should return None for empty port list."""
        result = find_display_port([])
        assert result is None

    def test_uses_system_ports_if_none_provided(self):
        """Function should use system ports if no list provided."""
        result = find_display_port()
        # Result is either a port object or None
        assert result is None or hasattr(result, 'device')


class TestOpenConnection:
    """Tests for open_connection function."""

    @patch('src.com_ports.serial.Serial')
    def test_opens_connection_with_correct_baud_rate(self, mock_serial):
        """Function should open connection with correct baud rate (115200)."""
        mock_connection = MagicMock()
        mock_serial.return_value = mock_connection
        
        result = open_connection("COM3")
        
        mock_serial.assert_called_once_with(
            "COM3",
            baudrate=DEFAULT_BAUD_RATE,
            timeout=DEFAULT_TIMEOUT
        )
        assert result is mock_connection

    @patch('src.com_ports.serial.Serial')
    def test_opens_connection_with_custom_baud_rate(self, mock_serial):
        """Function should accept custom baud rate."""
        mock_connection = MagicMock()
        mock_serial.return_value = mock_connection
        
        result = open_connection("COM3", baud_rate=9600)
        
        mock_serial.assert_called_once_with(
            "COM3",
            baudrate=9600,
            timeout=DEFAULT_TIMEOUT
        )

    @patch('src.com_ports.serial.Serial')
    def test_raises_exception_on_invalid_port(self, mock_serial):
        """Function should raise exception on invalid port."""
        mock_serial.side_effect = serial.SerialException("Port not found")
        
        with pytest.raises(serial.SerialException):
            open_connection("INVALID_PORT")

    @patch('src.com_ports.serial.Serial')
    def test_handles_timeout_parameter(self, mock_serial):
        """Function should handle timeout parameter."""
        mock_connection = MagicMock()
        mock_serial.return_value = mock_connection
        
        result = open_connection("COM3", timeout=5.0)
        
        mock_serial.assert_called_once_with(
            "COM3",
            baudrate=DEFAULT_BAUD_RATE,
            timeout=5.0
        )


class TestCloseConnection:
    """Tests for close_connection function."""

    def test_closes_open_connection(self):
        """Function should close open connection."""
        mock_connection = MagicMock()
        mock_connection.is_open = True
        
        close_connection(mock_connection)
        
        mock_connection.close.assert_called_once()

    def test_handles_already_closed_connection(self):
        """Function should handle already closed connection gracefully."""
        mock_connection = MagicMock()
        mock_connection.is_open = False
        
        # Should not raise exception
        close_connection(mock_connection)
        
        # close() should not be called if already closed
        mock_connection.close.assert_not_called()

    def test_handles_none_connection(self):
        """Function should handle None connection gracefully."""
        # Should not raise exception
        close_connection(None)
