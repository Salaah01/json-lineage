# JSON Linage (In Development)

## Introduction

JSON Linage is a tool that allows you to coverage JSON to JSONL (JSON Lines) format as well as iteratively parse JSON where the JSON contains a list of objects.

The underlying program is written in Rust and is built to feed one JSON object at a time to the parser. This allows for the parsing of very large JSON files that would otherwise not fit into memory.

Additionally, this project contains adapters for easy integration into other programming languages. Currently, there is only a Python adapter, but more are planned.
