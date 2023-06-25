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

    def __init__(self, filepath: str):
        self.bin_path = get_bin_path()
        self.file_path = filepath
        self._proc: _t.Optional[
            _t.Union[subprocess.Popen, asyncio.subprocess.Process]
        ] = None

    def __repr__(self) -> str:
        return (
            f"<{self.__class__.__name__} bin_path={self.bin_path} "
            f"file_path={self.file_path}>"
        )

    def kill_subprocess_proc(self) -> None:
        """Kill the subprocess process."""
        if self._proc is None:
            return

        for stream in ("stdout", "stderr"):
            stream = getattr(self._proc, stream)
            if hasattr(stream, "close"):
                stream.close()

        try:
            self._proc.terminate()
        except ProcessLookupError:
            pass
        self._proc = None


class BinaryReader(BaseBinaryReader):
    """Subprocess wrapper for the jsonl_converter binary."""

    def __iter__(self):
        return BinaryIterator(self.popen())

    def popen(self) -> subprocess.Popen:
        """Run the binary and return a Popen object."""
        self._proc = subprocess.Popen(
            [self.bin_path, self.file_path],
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            universal_newlines=True,
        )
        return self._proc


class BinaryIterator:
    """Iterator for the `BinaryReader` class."""

    def __init__(self, process: subprocess.Popen):
        self.process = process

    def __iter__(self):
        return self

    def __next__(self) -> str:
        self.raise_err_if_stderr()

        if self.process.stdout is None:
            raise StopIteration
        line = self.process.stdout.readline().strip()
        self.raise_err_if_stderr()
        if not line and self.process.poll() is not None:
            raise StopIteration

        return line

    def raise_err_if_stderr(self) -> None:
        """Raise an exception if the process has exited with a non-zero
        code.
        """
        if self.process.poll() is not None and self.process.poll() != 0:
            if self.process.stderr is None:
                raise BinaryExecutionException(
                    f"Process exited with code {self.process.poll()}"
                )
            else:
                raise BinaryExecutionException(self.process.stderr.read())


class AsyncBinaryReader(BaseBinaryReader):
    """Async subprocess wrapper for the jsonl_converter binary."""

    async def popen(self) -> asyncio.subprocess.Process:
        """Run the binary and return a Popen object."""
        self._proc = await asyncio.create_subprocess_exec(
            self.bin_path,
            self.file_path,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
        )
        return self._proc

    async def read_output(self, process: asyncio.subprocess.Process) -> str:
        if process.stdout is None:
            return ""

        line = await process.stdout.readline()
        if not line:
            return ""

        return line.decode().rstrip()

    def __aiter__(self):
        return AsyncBinaryIterator(self.popen())


class AsyncBinaryIterator:
    def __init__(self, process_coro: Coroutine):
        self.process_coro = process_coro
        self.process: asyncio.subprocess.Process | None = None

    async def __aiter__(self):
        return self

    async def __anext__(self) -> str:
        if self.process is None:
            self.process = await _t.cast(
                Awaitable[asyncio.subprocess.Process],
                self.process_coro,
            )
        output = await self.read_output(self.process)
        await self.raise_err_if_stderr()

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

    async def raise_err_if_stderr(self):
        if (
            self.process.returncode is not None
            and self.process.returncode != 0
        ):
            err = await self.process.stderr.read()
            if err:
                raise BinaryExecutionException(err)
            else:
                raise BinaryExecutionException(
                    f"Process exited with code {self.process.returncode}"
                )
