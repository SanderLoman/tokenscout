ifeq ($(OS),Windows_NT)
    FEATURES?=
else
    FEATURES?=jemalloc
endif

PROFILE ?= release

run:
	cargo run --features "$(FEATURES)" --release

build:
	cargo build --features "$(FEATURES)"

test:
	cargo test --features "$(FEATURES)"