"""Contains the public API for the JSON lineage adapter. The module should
aim to mimic the `json` module in the Python standard library as much as
possible for easy adoption.
"""

from .bin_interface import AsyncBinaryReader, BinaryReader

__all__ = [
    "load",
    "aload",
]


def load(fp: str, *, messy: bool = False) -> BinaryReader:
    """Return a `BinaryReader` object for the given file path."""
    return BinaryReader(fp, messy)


def aload(fp: str, *, messy: bool = False) -> AsyncBinaryReader:
    """Return an `AsyncBinaryReader` object for the given file path."""
    return AsyncBinaryReader(fp, messy)
