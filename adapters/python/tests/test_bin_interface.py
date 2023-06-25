import asyncio
import platform
import subprocess
import time
from types import SimpleNamespace
from typing import Callable, Union
from unittest import IsolatedAsyncioTestCase, TestCase
from unittest.mock import patch

from json_lineage import bin_interface
from json_lineage.exceptions import BinaryExecutionException

from .helpers import SAMPLE_DATA_PATH


class ReaderInstanceMixin:
    create_reader_instance: Callable[
        ..., Union[bin_interface.BinaryReader, bin_interface.AsyncBinaryReader]
    ]

    def setUp(self):
        self.reader = self.create_reader_instance()

    def tearDown(self):
        self.reader.kill_subprocess_proc()


class TestGetBinPath(TestCase):
    """Tests for the `get_bin_path` function."""

    @patch.object(platform, "system", return_value="Windows")
    def test_get_bin_path_as_windows(self, _):
        """Test that the `get_bin_path` function returns the correct path on
        Windows.
        """
        self.assertTrue(
            bin_interface.get_bin_path().endswith("jsonl_converter.exe"),
        )

    @patch.object(platform, "system", return_value="Linux")
    def test_get_bin_path_as_linux(self, _):
        """Test that the `get_bin_path` function returns the correct path on
        Linux.
        """
        self.assertTrue(
            bin_interface.get_bin_path().endswith("jsonl_converter"),
        )


class TestBaseBinaryReader(TestCase):
    def test__init__works(self):
        """Test that the `BinaryReader` class can be instantiated."""
        bin_interface.BinaryReader("filepath")

    def test_repr(self):
        """Test that the `__repr__` method returns a string."""
        reader = bin_interface.BinaryReader("filepath")
        self.assertIsInstance(repr(reader), str)

    def test_kill_subprocess_proc_returns_if_no_proc(self):
        """Test that the `kill_subprocess_proc` method returns if there is no
        subprocess.
        """
        reader = bin_interface.BinaryReader("filepath")
        reader.kill_subprocess_proc()

    def test_kill_subprocess_proc_closes_files_and_terminates_proc(self):
        """Test that the `kill_subprocess_proc` method closes the files and
        terminates the subprocess.
        """
        reader = bin_interface.BinaryReader(SAMPLE_DATA_PATH)
        proc = reader.popen()
        reader.kill_subprocess_proc()
        self.assertTrue(proc.stdout.closed)
        self.assertTrue(proc.stderr.closed)
        time.sleep(0.01)
        self.assertIsNotNone(proc.poll())


class TestBinaryReader(ReaderInstanceMixin, TestCase):
    """Tests for the `BinaryReader` class."""

    @staticmethod
    def create_reader_instance():
        return bin_interface.BinaryReader(SAMPLE_DATA_PATH)

    def test_popen_returns_popen(self):
        """Test that the `popen` method returns a `subprocess.Popen` object."""
        proc = self.reader.popen()
        self.assertIsInstance(proc, subprocess.Popen)
        proc.communicate()

    def test_ppopen_readable_stdout(self):
        """Test that the `popen` method returns a `subprocess.Popen` object
        with a readable stdout.
        """
        proc = self.reader.popen()
        self.assertIsNotNone(proc.stdout.readline())
        proc.communicate()

    def test_iter(self):
        """Test that the `__iter__` method returns a `BinaryIterator`
        object.
        """
        self.assertIsInstance(iter(self.reader), bin_interface.BinaryIterator)
        self.reader._proc.communicate()

    def test_iter_next_valid(self):
        """Test that the `__next__` method iterates over the binary stdout
        correctly.
        """
        iterator = iter(self.reader)
        self.assertEqual(next(iterator), '{"a": {"B": 1},"b": 2}')
        self.assertEqual(next(iterator), '{"a": 1,"b": 2}')
        with self.assertRaises(StopIteration):
            next(iterator)

    def test_raises_err_if_non_0_return_code_with_stderr_from_bin(self):
        """Test that the `__next__` method raises a `BinaryExecutionException`
        if the binary returns a non-zero return code and there is stderr from
        the binary.
        """
        reader = bin_interface.BinaryReader("invalid_path")
        with self.assertRaises(BinaryExecutionException):
            next(iter(reader))
        reader.kill_subprocess_proc()

    def test_raises_err_if_non_0_return_code_no_stderr_from_bin(self):
        """Test that the `__next__` method raises a `BinaryExecutionException`
        if the binary returns a non-zero return code and there is no stderr
        from the binary.
        """
        reader = bin_interface.BinaryReader("invalid_path")
        with self.assertRaises(BinaryExecutionException):
            next(iter(reader))
        reader.kill_subprocess_proc()


class TestBinaryIterator(TestCase):
    """Tests for the `BinaryIterator` class."""

    def test__iter__returns_instance_of_self(self):
        """Test that the `__iter__` method returns an instance of itself."""
        iterator = bin_interface.BinaryIterator(None)
        self.assertIs(iter(iterator), iterator)

    def test__next__raises_stop_iter_if_no_stdout(self):
        """Test that the `__next__` method raises a `StopIteration` if there
        is no stdout.
        """
        iterator = bin_interface.BinaryIterator(
            SimpleNamespace(stdout=None, stderr=None, poll=lambda: 0)
        )
        with self.assertRaises(StopIteration):
            next(iterator)


class TestAsyncBinaryReader(ReaderInstanceMixin, IsolatedAsyncioTestCase):
    """Tests for the `AsyncBinaryReader` class."""

    @staticmethod
    def create_reader_instance():
        return bin_interface.AsyncBinaryReader(SAMPLE_DATA_PATH)

    async def test_ppopen_returns_popen(self):
        """Test that the `popen` method returns the correct type of
        instance.
        """
        proc = await self.reader.popen()
        self.assertIsInstance(proc, asyncio.subprocess.Process)

    async def test_raises_err_if_non_0_return_code(self):
        """Test that the `__anext__` method raises a `BinaryExecutionException`
        if the binary returns a non-zero return code.
        """

        async def fn():
            reader = bin_interface.AsyncBinaryReader("invalid_path")
            with self.assertRaises(BinaryExecutionException):
                async for _ in reader:
                    pass

        await asyncio.wait_for(fn(), timeout=0.1)

    async def test__aiter__returns_async_binary_iterator(self):
        """Test that the `__aiter__` method returns an `AsyncBinaryIterator`
        object.
        """
        aiter_ = self.reader.__aiter__()
        await aiter_.__anext__()

        self.assertIsInstance(aiter_, bin_interface.AsyncBinaryIterator)

    async def test__anext__valid(self):
        """Test that the `__anext__` method iterates over the binary stdout
        correctly.
        """
        iterator = self.reader.__aiter__()
        self.assertEqual(await iterator.__anext__(), '{"a": {"B": 1},"b": 2}')
        self.assertEqual(await iterator.__anext__(), '{"a": 1,"b": 2}')
        with self.assertRaises(StopAsyncIteration):
            await iterator.__anext__()


class TestAsyncBinaryIterator(ReaderInstanceMixin, IsolatedAsyncioTestCase):
    """Tests for the `AsyncBinaryIterator` class."""

    @staticmethod
    def create_reader_instance():
        return bin_interface.AsyncBinaryReader(SAMPLE_DATA_PATH)

    async def test_read_output_reads_stdout_til_exhaustion(self):
        """Test that the `read_output` method reads the stdout until it is
        exhausted.
        """
        proc = await self.reader.popen()
        self.assertEqual(
            await self.reader.read_output(proc),
            '{"a": {"B": 1},"b": 2}',
        )
        self.assertEqual(
            await self.reader.read_output(proc),
            '{"a": 1,"b": 2}',
        )
        self.assertEqual(await self.reader.read_output(proc), "")
        self.reader.kill_subprocess_proc()

    async def test_read_output_returns_empty_string_if_none_stdout(self):
        """Test that the `read_output` method returns an empty string if the
        stdout is `None`.
        """
        proc = await asyncio.create_subprocess_exec(
            "echo",
            "test",
            stdout=None,
        )
        self.assertEqual(await self.reader.read_output(proc), "")
        self.reader.kill_subprocess_proc()

    async def test__aiter__returns_instance_of_self(self):
        """Test that the `__aiter__` method returns an instance of itself."""
        iterator = bin_interface.AsyncBinaryIterator(None)
        self.assertIs(await iterator.__aiter__(), iterator)
