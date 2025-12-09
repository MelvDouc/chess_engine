.PHONY: run test flamegraph

PROJECT_NAME = chess2

run:
	@cargo run

test:
	@cargo test

flamegraph:
	@cargo flamegraph --bin $(PROJECT_NAME)