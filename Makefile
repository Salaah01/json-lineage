SHELL := /bin/bash

COMPILED_WINDOWS_EXE_PATH = jsonl_converter/target/x86_64-pc-windows-gnu/release/jsonl_converter.exe
COMPILED_LINUX_PATH = jsonl_converter/target/release/jsonl_converter

ADAPTERS_PYTHON_BIN_PATH = adapters/python/json_lineage/bin

compile-windows:
	cd jsonl_converter && cargo build --release --target x86_64-pc-windows-gnu

copy-windows-executable:
	cp $(COMPILED_WINDOWS_EXE_PATH) $(ADAPTERS_PYTHON_BIN_PATH)/jsonl_converter.exe

compile-linux:
	cd jsonl_converter && cargo build --release

copy-linux-executable:
	cp $(COMPILED_LINUX_PATH) $(ADAPTERS_PYTHON_BIN_PATH)/jsonl_converter

