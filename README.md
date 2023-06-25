# JSON Lineage
## Introduction

JSON Linage is a tool that allows you to convert JSON to JSONL (JSON Lines) format as well as iteratively parse JSON where the JSON contains a list of objects.

The underlying program is written in Rust and is built to feed one JSON object at a time to the parser. This allows for the parsing of very large JSON files that would otherwise not fit into memory.

Additionally, this project contains adapters for easy integration into other programming languages. Currently, there is only a Python adapter, but more are planned.

## Adapters

### Python

The Python adapter is a wrapper around the underlying Rust program. It allows for easy integration into Python programs.
It is designed to feel a similar to the built-in `json` module in Python.

The following functionality is provided:

* `load` - Generate an iterator that returns each object in a JSON file.
* `aload` - Generates an asynchronous iterator that returns each object in a JSON file.

A CLI is also provided for easy conversion of JSON files to JSONL files.
For information on how to use the CLI, run: `python -m json_lineage --help`.

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


### Under the Hood

The underlying program is written in Rust. The full documentation for the underlying program can be found [here](https://salaah01.github.io/json-lineage/docs/cargo/jsonl_converter/index.html).
