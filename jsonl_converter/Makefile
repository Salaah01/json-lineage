SHELL := /bin/bash

.PHONY: comoile-windows
compile-windows:
	cargo build --release --target x86_64-pc-windows-gnu

.PHONY: compile-linux
compile-linux:
	cargo build --release

.PHONY: compile-all
compile-all:
	make compile-windows
	make compile-linux

.PHONY: build-docs
build-docs:
	cargo doc --no-deps
