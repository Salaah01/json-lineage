import builtins
import sys
import tempfile
from unittest import TestCase
from unittest.mock import patch

from json_lineage import cli, consumer

from .helpers import SAMPLE_DATA_PATH


class TestParseArgs(TestCase):
    """Tests for the `parse_args` function."""

    def test_args_are_parsed(self):
        args = ["file.json", "-o", "file.jsonl"]
        with patch.object(sys, "argv", ["json_lineage"] + args):
            parsed_args = cli.parse_args()
            self.assertEqual(parsed_args.filepath, args[0])
            self.assertEqual(parsed_args.output_file, args[2])


class TestPrintLines(TestCase):
    """Tests for the `print_lines` function."""

    def test_print_lines(self):
        """Test that the `print_lines` function prints the lines from the
        given reader to stdout.
        """
        reader = consumer.BinaryReader(SAMPLE_DATA_PATH)
        with patch.object(builtins, "print") as mock_print:
            cli.print_lines(reader)
            self.assertEqual(mock_print.call_count, 2)

    def test_write_lines(self):
        """Test that the `write_lines` function writes the lines from the
        given reader to the given filepath.
        """
        reader = consumer.BinaryReader(SAMPLE_DATA_PATH)
        with tempfile.NamedTemporaryFile() as f:
            cli.write_lines(reader, f.name)
            self.assertEqual(f.read().count(b"\n"), 2)


class TestMain(TestCase):
    """Tests for the `main` function."""

    @patch.object(sys, "argv", ["json_lineage", SAMPLE_DATA_PATH])
    def test_main_with_no_output_file_prints(self):
        """Test that the `main` function prints the lines from the given
        reader to stdout when no output file is given.
        """
        with patch.object(cli, "print_lines") as mock_print_lines:
            cli.main()
            self.assertEqual(mock_print_lines.call_count, 1)

    @patch.object(
        sys,
        "argv",
        ["json_lineage", SAMPLE_DATA_PATH, "-o", "file.jsonl"],
    )
    def test_main_with_output_file_writes(self):
        """Test that the `main` function writes the lines from the given
        reader to the given filepath when an output file is given.
        """
        with patch.object(cli, "write_lines") as mock_write_lines:
            cli.main()
            self.assertEqual(mock_write_lines.call_count, 1)
