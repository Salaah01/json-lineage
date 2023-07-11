# JSON Lineage

## Table of Contents
- [JSON Lineage](#json-lineage)
  - [Table of Contents](#table-of-contents)
  - [Introduction](#introduction)
  - [Adapters](#adapters)
    - [Python](#python)
      - [Why not Just Use Python's `json` Library?](#why-not-just-use-pythons-json-library)
      - [Functionality](#functionality)
      - [Performance Comparison](#performance-comparison)
      - [Installation](#installation)
      - [Usage](#usage)
        - [Iterating over a JSON file](#iterating-over-a-json-file)
        - [Iterating over a JSON file asynchronously](#iterating-over-a-json-file-asynchronously)
        - [Poorly Formatted JSON](#poorly-formatted-json)
  - [Under the Hood](#under-the-hood)


## Introduction

JSON Linage is a tool that allows you to convert JSON to JSONL (JSON Lines) format as well as iteratively parse JSON where the JSON contains a list of objects.

The underlying program is written in Rust and is built to feed one JSON object at a time to the parser. This allows for the parsing of very large JSON files that would otherwise not fit into memory. In addition to saving memory, this program is capable of parsing JSON files faster than the built-in Python JSON parser as the file size increases.

Additionally, this project contains adapters for easy integration into other programming languages. Currently, there is only a Python adapter, but more are planned.

## Adapters

### Python

The Python adapter is a wrapper around the underlying Rust program, providing seamless integration into Python applications. It is designed to have a similar feel to Python's built-in json module.

#### Why not Just Use Python's `json` Library?

You might wonder why you would choose to use this library instead of Python's built-in JSON library. The answer depends on your specific use case.

If you are parsing a small JSON file, Python's JSON library is likely sufficient and performs well. However, when dealing with very large JSON files that exceed the available memory, JSON Lineage offers significant benefits.

Python's JSON library is written in C and is highly optimised for speed. However, it loads the entire JSON file into memory, making it unsuitable for parsing very large JSON files. This is where JSON Lineage shines.

JSON Lineage is specifically designed to parse very large JSON files that would not fit into memory. It achieves this by parsing the JSON file one object at a time.

#### Functionality

The following functionality is provided:

* `load` - Generate an iterator that returns each object in a JSON file.
* `aload` - Generates an asynchronous iterator that returns each object in a JSON file.

A CLI is also provided for easy conversion of JSON files to JSONL files.
For information on how to use the CLI, run: `python -m json_lineage --help`.

#### Performance Comparison

The following graphs compare the speed and memory usage of Python's JSON library vs JSON Lineage.

The benchmarks show that up to a file size of 500MB, the speed difference is negligible. However, already at this point, Python requires almost 2GB of memory to parse the JSON file, while JSON Lineage only requires 1.5MB.

As the file size continues to grow, Python's JSON library continues to be faster, but the memory usage continues to grow at a linear rate. JSON Lineage, in contrast, continues to use the same amount of memory.


![Benchmark of difference in time as file size grows](https://github.com/Salaah01/json-lineage/raw/master/docs/benchmark/time_diff_chart.png)

![Benchmark of difference in memory as file size grows](https://github.com/Salaah01/json-lineage/raw/master/docs/benchmark/mem_diff_chart.png)

#### Installation

```bash
pip install json-lineage
```

#### Usage

##### Iterating over a JSON file

```python
from json_lineage import load

jsonl_iter = load("path/to/file.json")

for obj in jsonl_iter:
    do_something(obj)
```

##### Iterating over a JSON file asynchronously

```python
import asyncio
from random import randint
from json_lineage import aload

jsonl_iter = aload("path/to/file.json")


async def do_something(i):
    await asyncio.sleep(randint(1, 2))
    print(i)


async def main():
    tasks = []
    async for i in async_iter:
        tasks.append(asyncio.create_task(do_something(i)))
    
    await asyncio.gather(*tasks)


asyncio.run(main())
```

##### Poorly Formatted JSON

When parsing a JSON file, the program will assume that the JSON file is well formatted. If the JSON file is not well formatted, then you can provide a `messy=True` argument to either the sync or async load:

```python
from json_lineage import load

jsonl_iter = load("path/to/file.json", messy=True)


for obj in jsonl_iter:
    do_something(obj)
```

This will cause the program to output the same results. However, how it parses the JSON file will be different. Using this option will cause the program to be slower, but it will be able to parse JSON files that are not well formatted.

If you are using the CLI, then you can use the `--messy` flag to achieve the same result.

## Under the Hood

The underlying program is written in Rust. The full documentation for the underlying program can be found [here](https://salaah01.github.io/json-lineage/docs/cargo/jsonl_converter/index.html).
