import asyncio
import platform
import subprocess
from types import SimpleNamespace
from unittest import IsolatedAsyncioTestCase, TestCase
from unittest.mock import patch

from json_lineage import consumer
from json_lineage.exceptions import BinaryExecutionException

from .helpers import SAMPLE_DATA_PATH


class TestGetBinPath(TestCase):
    """Tests for the `get_bin_path` function."""

    @patch.object(platform, "system", return_value="Windows")
    def test_get_bin_path_as_windows(self, _):
        """Test that the `get_bin_path` function returns the correct path on
        Windows.
        """
        self.assertTrue(
            consumer.get_bin_path().endswith("jsonl_converter.exe"),
        )

    @patch.object(platform, "system", return_value="Linux")
    def test_get_bin_path_as_linux(self, _):
        """Test that the `get_bin_path` function returns the correct path on
        Linux.
        """
        self.assertTrue(
            consumer.get_bin_path().endswith("jsonl_converter"),
        )


class TestBaseBinaryReader(TestCase):
    bin_path = consumer.get_bin_path()

    def test__init__works(self):
        """Test that the `BinaryReader` class can be instantiated."""
        consumer.BinaryReader("a", "b")

    def test_repr(self):
        """Test that the `__repr__` method returns a string."""
        reader = consumer.BinaryReader("a", "b")
        self.assertIsInstance(repr(reader), str)


class TestBinaryReader(TestCase):
    """Tests for the `BinaryReader` class."""

    bin_path = consumer.get_bin_path()

    def create_reader_instance(self):
        return consumer.BinaryReader(
            self.bin_path,
            SAMPLE_DATA_PATH,
        )

    def test_popen_returns_popen(self):
        """Test that the `popen` method returns a `subprocess.Popen` object."""
        reader = self.create_reader_instance()
        proc = reader.popen()
        self.assertIsInstance(proc, subprocess.Popen)
        proc.communicate()

    def test_ppopen_readable_stdout(self):
        """Test that the `popen` method returns a `subprocess.Popen` object
        with a readable stdout.
        """
        reader = self.create_reader_instance()
        proc = reader.popen()
        self.assertIsNotNone(proc.stdout.readline())
        proc.communicate()

    def test_ppopen_raises_err_if_stderr_from_bin(self):
        """Test that the `popen` method raises a `BinaryExecutionException`
        if the binary returns a stderr.
        """
        reader = consumer.BinaryReader(self.bin_path, "invalid_path")
        with self.assertRaises(BinaryExecutionException):
            reader.popen()

    def test_iter(self):
        """Test that the `__iter__` method returns a `BinaryIterator`
        object.
        """
        reader = self.create_reader_instance()
        self.assertIsInstance(iter(reader), consumer.BinaryIterator)

    def test_iter_next_valid(self):
        """Test that the `__next__` method iterates over the binary stdout
        correctly.
        """
        reader = self.create_reader_instance()
        iterator = iter(reader)
        self.assertEqual(next(iterator), '{"a": {"B": 1},"b": 2}')
        self.assertEqual(next(iterator), '{"a": 1,"b": 2}')
        with self.assertRaises(StopIteration):
            next(iterator)


class TestBinaryIterator(TestCase):
    """Tests for the `BinaryIterator` class."""

    def test__iter__returns_instance_of_self(self):
        """Test that the `__iter__` method returns an instance of itself."""
        iterator = consumer.BinaryIterator(None)
        self.assertIs(iter(iterator), iterator)

    def test__next__raises_stop_iter_if_no_stdout(self):
        """Test that the `__next__` method raises a `StopIteration` if there
        is no stdout.
        """
        iterator = consumer.BinaryIterator(SimpleNamespace(stdout=None))
        with self.assertRaises(StopIteration):
            next(iterator)


class TestAsyncBinaryReader(IsolatedAsyncioTestCase):
    """Tests for the `AsyncBinaryReader` class."""

    bin_path = consumer.get_bin_path()

    def create_reader_instance(self):
        return consumer.AsyncBinaryReader(
            self.bin_path,
            SAMPLE_DATA_PATH,
        )

    async def test_ppopen_returns_popen(self):
        """Test that the `popen` method returns the correct type of
        instance.
        """
        reader = self.create_reader_instance()
        proc = await reader.popen()
        self.assertIsInstance(proc, asyncio.subprocess.Process)
        proc.communicate()

    async def test_ppopen_raises_err_if_stderr_from_bin(self):
        """Test that the `popen` method raises a `BinaryExecutionException`
        if the binary returns a stderr.
        """
        reader = consumer.AsyncBinaryReader(self.bin_path, "invalid_path")
        with self.assertRaises(BinaryExecutionException):
            await reader.popen()

    def test__aiter__returns_async_binary_iterator(self):
        """Test that the `__aiter__` method returns an `AsyncBinaryIterator`
        object.
        """
        reader = self.create_reader_instance()
        self.assertIsInstance(
            reader.__aiter__(),
            consumer.AsyncBinaryIterator,
        )

    async def test__anext__valid(self):
        """Test that the `__anext__` method iterates over the binary stdout
        correctly.
        """
        reader = self.create_reader_instance()
        iterator = reader.__aiter__()
        self.assertEqual(await iterator.__anext__(), '{"a": {"B": 1},"b": 2}')
        self.assertEqual(await iterator.__anext__(), '{"a": 1,"b": 2}')
        with self.assertRaises(StopAsyncIteration):
            await iterator.__anext__()


class TestAsyncBinaryIterator(IsolatedAsyncioTestCase):
    """Tests for the `AsyncBinaryIterator` class."""

    bin_path = consumer.get_bin_path()

    def create_reader_instance(self):
        return consumer.AsyncBinaryReader(
            self.bin_path,
            SAMPLE_DATA_PATH,
        )

    async def test_read_output_reads_stdout_til_exhaustion(self):
        """Test that the `read_output` method reads the stdout until it is
        exhausted.
        """
        reader = self.create_reader_instance()
        proc = await reader.popen()
        self.assertEqual(
            await reader.read_output(proc),
            '{"a": {"B": 1},"b": 2}',
        )
        self.assertEqual(await reader.read_output(proc), '{"a": 1,"b": 2}')
        self.assertEqual(await reader.read_output(proc), "")
        proc.communicate()

    async def test_read_output_returns_empty_string_if_none_stdout(self):
        """Test that the `read_output` method returns an empty string if the
        stdout is `None`.
        """
        reader = self.create_reader_instance()
        proc = await asyncio.create_subprocess_exec(
            "echo",
            "test",
            stdout=None,
        )
        self.assertEqual(await reader.read_output(proc), "")

    async def test__aiter__returns_instance_of_self(self):
        """Test that the `__aiter__` method returns an instance of itself."""
        iterator = consumer.AsyncBinaryIterator(None)
        self.assertIs(await iterator.__aiter__(), iterator)
