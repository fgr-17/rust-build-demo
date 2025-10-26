.PHONY: format lint test doc build cov

include coverage.mk

all: format lint cov doc

format:
	cargo fmt --all

lint:
	cargo clippy -- -D warnings

test:
	cargo test

doc:
	cargo doc

build:
	cargo build

run:
	cargo run
