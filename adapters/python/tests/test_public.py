from unittest import TestCase

from json_lineage import aload, load
from json_lineage.bin_interface import AsyncBinaryReader, BinaryReader


class TestPublic(TestCase):
    """Tests for the `public` module."""

    def test_load_returns_binary_reader(self):
        """Test that `load` returns a `BinaryReader` object."""
        self.assertIsInstance(load("foo"), BinaryReader)

    def test_aload_returns_async_binary_reader(self):
        """Test that `aload` returns an `AsyncBinaryReader` object."""
        self.assertIsInstance(aload("foo"), AsyncBinaryReader)
