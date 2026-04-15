.PHONY: build test lint check release install clean

build:
	cargo build

test:
	cargo test

lint:
	cargo clippy

check: lint test

release:
	cargo build --release

install:
	cargo install --path .

clean:
	cargo clean
