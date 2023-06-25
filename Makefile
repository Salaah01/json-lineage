SHELL := /bin/bash

COMPILED_WINDOWS_EXE_PATH = jsonl_converter/target/x86_64-pc-windows-gnu/release/jsonl_converter.exe
COMPILED_LINUX_PATH = jsonl_converter/target/release/jsonl_converter

ADAPTERS_PYTHON_BIN_PATH = adapters/python/json_lineage/bin


.PHONY: build
build:
	cd jsonl_converter && make compile-all
	make copy-executables
	cd adapters/python && make build


.PHONY: copy-executables
copy-executables:
	make copy-windows-executable
	make copy-linux-executable

.PONY: copy-windows
compile-windows:
	cd jsonl_converter && cargo build --release --target x86_64-pc-windows-gnu

.PHONY: copy-windows-executable
copy-windows-executable:
	cp $(COMPILED_WINDOWS_EXE_PATH) $(ADAPTERS_PYTHON_BIN_PATH)/jsonl_converter.exe

.PHONY: compile-linux
compile-linux:
	cd jsonl_converter && cargo build --release

.PHONY: copy-linux-executable
copy-linux-executable:
	cp $(COMPILED_LINUX_PATH) $(ADAPTERS_PYTHON_BIN_PATH)/jsonl_converter

.PHONY: build-docs
build-docs:
	cd jsonl_converter && make build-docs
	rm -rf docs/cargo
	mkdir -p docs/cargo
	cp -r jsonl_converter/target/doc/* docs/cargo