"""
Tests for COM port enumeration functionality.
"""
import pytest
from src.com_ports import list_com_ports, format_port_info


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
