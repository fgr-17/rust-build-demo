.PHONY: format lint test doc build

include genproto.mk
include genflatb.mk
include coverage.mk

all: format lint cov doc
clean: cclean
	-@$(MAKE) cleanpb 2>/dev/null || true
	-@$(MAKE) cleanfb 2>/dev/null || true

format:
	cargo fmt --all

lint:
	cargo clippy -- -D warnings

cclean:
	cargo clean

test:
	cargo test

doc:
	cargo doc

build:
	cargo build

run:
	cargo run

