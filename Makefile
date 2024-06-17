ifeq ($(OS),Windows_NT)
    FEATURES?=
else
    FEATURES?=jemalloc
endif

PROFILE ?= release

run:
	cargo run --features "$(FEATURES)"

build:
	cargo build --features "$(FEATURES)"

test:
	cargo test --features "$(FEATURES)"