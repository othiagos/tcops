# Exemple: make run input=data/instances/inst.tcops mode=heuristic show=1 save=1

EXE = target/release/tcops
EXE_DEV = target/debug/tcops
CLI_ARGS = \
	$(if $(input),          --input=$(input)) \
	$(if $(mode),           --mode=$(mode)) \
	$(if $(solver),         --solver=$(solver)) \
	$(if $(max_iterations), --max-iterations=$(max_iterations)) \
	$(if $(max_shaking_intensity), --max-shaking-intensity=$(max_shaking_intensity)) \
	$(if $(show),           --show) \
	$(if $(save),           --save)

all: format test-release build

check:
	@cargo check

build-dev:
	@cargo build

build:
	@cargo build --release

run-dev:
	@cargo run -- $(CLI_ARGS) || true

run:
	@cargo run --release -- $(CLI_ARGS) || true

format:
	@cargo fmt

lint:
	@cargo clippy -- -D warnings

test:
	@cargo test

test-release:
	@cargo test --release

clean:
	@cargo clean

help:
	@cargo run -- --help

version:
	@cargo run -- --version

.PHONY: all check build build-dev run format lint test unit-test clean doc help version