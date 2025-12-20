.PHONY: run build test flamegraph

PROJECT_NAME = chess

run:
	@cargo run

build:
	@cargo build

test:
	@cargo test

flamegraph:
	@cargo flamegraph --bin $(PROJECT_NAME)