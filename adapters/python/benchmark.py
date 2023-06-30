"""This script is used to benchmark the performance of the Rust library
compared to the Python's built-in JSON library.

It compares the time and memory usage of the two libraries when loading a JSON
file.
"""

import asyncio
import json
import os
import resource
import timeit

from json_lineage import aload, load

FP = os.path.join(
    os.path.dirname(os.path.realpath(__file__)),
    "..",
    "..",
    "sample_data",
    "648mb_sample.json",
)


def using_rust_lib():
    for i in load(FP):
        i


def using_python_lib():
    for i in json.load(open(FP)):
        i


async def using_rust_lib_async():
    async for i in aload(FP):
        i


def async_main():
    asyncio.run(using_rust_lib_async())


def benchmark(fn):
    print(f"{'BENCHMARKING:'.ljust(15)}{fn.__name__}")
    start_mem = resource.getrusage(resource.RUSAGE_SELF).ru_maxrss
    exec_time = timeit.timeit(fn, number=1)
    print(f"{'TIME:'.ljust(15)}{exec_time}s")
    delta_mb = (
        resource.getrusage(resource.RUSAGE_SELF).ru_maxrss - start_mem
    ) / 1024
    print(f"{'MEMRORY USAGE:'.ljust(15)}{delta_mb} MB\n")
    return exec_time, delta_mb


if __name__ == "__main__":
    rs_time, rs_mem = benchmark(using_rust_lib)
    py_time, py_mem = benchmark(using_python_lib)
    print(f"{py_time}|{rs_time}|{py_mem}|{rs_mem}")
