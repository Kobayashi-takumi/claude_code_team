.PHONY: build test run clean fmt lint check

build:
	cargo build --release

test:
	cargo test

run:
	cargo run

clean:
	cargo clean

fmt:
	cargo fmt

lint:
	cargo clippy -- -D warnings

check: fmt lint test
	@echo "All checks passed!"

install:
	cargo install --path .

dev:
	cargo watch -x "run"