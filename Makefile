ifeq ($(OS),Windows_NT)
    FEATURES?=
else
    FEATURES?=jemalloc
endif

PROFILE ?= release

install:
	cargo install --path tokenscout --force --locked \
		--features "$(FEATURES)" \
		--profile "$(PROFILE)"

run:
	cargo run --features "$(FEATURES)" --release

build:
	cargo build --features "$(FEATURES)"

test:
	cargo test --features "$(FEATURES)"