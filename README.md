# JSON Lineage

## Table of Contents
- [JSON Lineage](#json-lineage)
  - [Table of Contents](#table-of-contents)
  - [Introduction](#introduction)
  - [Adapters](#adapters)
    - [Python](#python)
      - [Why not Just Use Python's `json` Library?](#why-not-just-use-pythons-json-library)
      - [Functionality](#functionality)
      - [Benchmarks](#benchmarks)
        - [32MB JSON file](#32mb-json-file)
        - [324MB JSON file](#324mb-json-file)
      - [Installation](#installation)
      - [Usage](#usage)
        - [Iterating over a JSON file](#iterating-over-a-json-file)
        - [Iterating over a JSON file asynchronously](#iterating-over-a-json-file-asynchronously)
  - [Under the Hood](#under-the-hood)


## Introduction

JSON Linage is a tool that allows you to convert JSON to JSONL (JSON Lines) format as well as iteratively parse JSON where the JSON contains a list of objects.

The underlying program is written in Rust and is built to feed one JSON object at a time to the parser. This allows for the parsing of very large JSON files that would otherwise not fit into memory.

Additionally, this project contains adapters for easy integration into other programming languages. Currently, there is only a Python adapter, but more are planned.

## Adapters

### Python

The Python adapter is a wrapper around the underlying Rust program. It allows for easy integration into Python programs.
It is designed to feel a similar to the built-in `json` module in Python.

#### Why not Just Use Python's `json` Library?

Given that Python already has a built-in JSON parser, you may be wondering why you would want to use this library.
The answer is, well, it depends.

If you are parsing a small JSON file, then you probably don't want to use this library.

Python's JSON library is written in C and is very fast. However, as it loads the entire JSON file into memory, it is not
suitable for parsing very large JSON files. This is where JSON Lineage comes in.

JSON Lineage is designed to parse very
large JSON files that would otherwise not fit into memory. It does this by parsing the JSON file one object at a time.

#### Functionality

The following functionality is provided:

* `load` - Generate an iterator that returns each object in a JSON file.
* `aload` - Generates an asynchronous iterator that returns each object in a JSON file.

A CLI is also provided for easy conversion of JSON files to JSONL files.
For information on how to use the CLI, run: `python -m json_lineage --help`.

#### Benchmarks

The following benchmarks where run comparing the performance of the Python JSON parser and JSON Lineage. These results should help you decide when Python's JSON parser is sufficient and when you should use JSON Lineage.

##### 32MB JSON file

| Library        | Time (s) | Memory (MB) |
| -------------- | -------- | ----------- |
| `json`         | 0.166    | 158.99      |
| `json_lineage` | 1.01     | 0.52        |

##### 324MB JSON file

| Library        | Time (s) | Memory (MB) |
| -------------- | -------- | ----------- |
| `json`         | 1.66     | 1580.46     |
| `json_lineage` | 10.06    | 0.71        |



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


## Under the Hood

The underlying program is written in Rust. The full documentation for the underlying program can be found [here](https://salaah01.github.io/json-lineage/docs/cargo/jsonl_converter/index.html).
