import asyncio
import os
import platform
import subprocess
import typing as _t
from collections.abc import Awaitable, Coroutine

from .exceptions import BinaryExecutionException

__all__ = [
    "BinaryReader",
    "AsyncBinaryReader",
]


def get_bin_path() -> str:
    """Get the path to the jsonl_converter binary."""
    bin_dir = os.path.join(os.path.dirname(os.path.realpath(__file__)), "bin")
    if platform.system() == "Windows":
        return os.path.join(bin_dir, "jsonl_converter.exe")
    else:
        return os.path.join(bin_dir, "jsonl_converter")


class BaseBinaryReader:
    """Base class for the `BinaryReader` and `AsyncBinaryReader` classes."""

    def __init__(self, bin_path: str, filepath: str):
        self.bin_path = bin_path
        self.file_path = filepath

    def __repr__(self) -> str:
        return (
            f"<{self.__class__.__name__} bin_path={self.bin_path} "
            f"file_path={self.file_path}>"
        )


class BinaryReader(BaseBinaryReader):
    """Subprocess wrapper for the jsonl_converter binary."""

    def __iter__(self):
        return BinaryIterator(self.popen())

    def popen(self) -> subprocess.Popen:
        """Run the binary and return a Popen object."""
        proc = subprocess.Popen(
            [self.bin_path, self.file_path],
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
        )
        if proc.stderr:
            err = proc.stderr.read()
        if err:
            raise BinaryExecutionException(
                f"Error in subprocess: {err.decode()}",
            )
        return proc


class BinaryIterator:
    """Iterator for the `BinaryReader` class."""

    def __init__(self, process: subprocess.Popen):
        self.process = process

    def __iter__(self):
        return self

    def __next__(self) -> str:
        if self.process.stdout is None:
            raise StopIteration
        line = self.process.stdout.readline().strip().decode()
        if not line:
            raise StopIteration

        return line


class AsyncBinaryReader(BaseBinaryReader):
    """Async subprocess wrapper for the jsonl_converter binary."""

    async def popen(self) -> asyncio.subprocess.Process:
        """Run the binary and return a Popen object."""
        proc = await asyncio.create_subprocess_exec(
            self.bin_path,
            self.file_path,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
        )
        err = await proc.stderr.read()  # type: ignore
        if err:
            raise BinaryExecutionException(err)
        return proc

    async def read_output(self, process: asyncio.subprocess.Process) -> str:
        if process.stdout is None:
            return ""

        line = await process.stdout.readline()
        if not line:
            return ""

        return line.decode().rstrip()

    def __aiter__(self):
        return AsyncBinaryIterator(self.popen())


class AsyncBinaryIterator(BaseBinaryReader):
    def __init__(self, process_coro: Coroutine):
        self.process_coro = process_coro
        self.process: asyncio.subprocess.Process | None = None

    async def __aiter__(self):
        return self

    async def __anext__(self) -> str:
        if not self.process:
            self.process = await _t.cast(
                Awaitable[asyncio.subprocess.Process],
                self.process_coro,
            )
        output = await self.read_output(self.process)
        if not output:
            raise StopAsyncIteration

        return output

    async def read_output(self, process: asyncio.subprocess.Process) -> str:
        if process.stdout is None:
            return ""
        line = await process.stdout.readline()
        if not line:
            return ""
        return line.decode().rstrip()


# async def async_test():
#     bin_path = get_bin_path()
#     fp = "/home/salaah/json-lineage/sample_data/sample.json"
#     reader = AsyncBinaryReader(bin_path, fp)
#     async for line in reader:
#         print(line)


# if __name__ == "__main__":
#     bin_path = get_bin_path()
#     fp = "/home/salaah/json-lineage/sample_data/sample.json"
#     reader = BinaryReader(bin_path, fp)
#     for line in reader:
#         print(line)
# loop = asyncio.get_event_loop()
# loop.run_until_complete(async_test())
# loop.close()
