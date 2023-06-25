"""This script is used to benchmark the performance of the Rust library
compared to the Python's built-in JSON library.

It compares the time and memory usage of the two libraries when loading a JSON
file.
"""

import json
import os
import resource
import timeit

from json_lineage import load

FP = os.path.join(
    os.path.dirname(os.path.realpath(__file__)),
    "..",
    "..",
    "sample_data",
    "32mb_sample.json",
)


def using_rust_lib():
    for i in load(FP):
        i


def using_python_lib():
    for i in json.load(open(FP)):
        i


def benchmark(fn):
    print(f"{'BENCHMARKING:'.ljust(15)}{fn.__name__}")
    start_mem = resource.getrusage(resource.RUSAGE_SELF).ru_maxrss
    print(f"{'TIME:'.ljust(15)}{timeit.timeit(fn, number=1)}s")
    delta = resource.getrusage(resource.RUSAGE_SELF).ru_maxrss - start_mem
    print(f"{'MEMRORY USAGE:'.ljust(15)}{delta/1024} MB\n")


if __name__ == "__main__":
    benchmark(using_rust_lib)
    benchmark(using_python_lib)
