"""This module contains the command line interface for the json_lineage
adapter.
"""

import argparse

from .bin_interface import BinaryReader


def parse_args() -> argparse.Namespace:
    """Parses the command line arguments."""
    parser = argparse.ArgumentParser(
        prog="python -m json_lineage",
        description="Read and convert JSON to JSONL (JSON Lines) format.",
    )
    parser.add_argument(
        "filepath",
        type=str,
        help="Path to the JSON file to read.",
    )
    parser.add_argument(
        "--output-file",
        "-o",
        type=str,
        help="Path to the output file.",
    )

    return parser.parse_args()


def print_lines(reader: BinaryReader) -> None:
    """Prints the lines from the given reader to stdout."""
    for line in reader:
        print(line)


def write_lines(reader: BinaryReader, filepath: str) -> None:
    """Writes the lines from the given reader to the given filepath."""
    with open(filepath, "w") as f:
        for line in reader:
            f.write(line + "\n")


def main() -> None:
    """The main entrypoint for the json_lineage adapter when run as a
    module.
    """
    args = parse_args()
    reader = BinaryReader(args.filepath)
    if args.output_file:
        write_lines(reader, args.output_file)
    else:
        print_lines(reader)
