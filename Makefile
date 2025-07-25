PROJECT = ascii-tactics
TARGET_DIR = target
BUILD_FLAGS = --release

# -----------------------------
# Build Targets
# -----------------------------

.PHONY: all build run clean

all: build

build:
	cargo build $(BUILD_FLAGS)

run:
	cargo run

clean:
	cargo clean

# -----------------------------
# Cross Compile
# -----------------------------

.PHONY: build-linux build-windows

build-linux:
	cargo build --target x86_64-unknown-linux-gnu $(BUILD_FLAGS)

build-windows:
	cross build --target x86_64-pc-windows-gnu $(BUILD_FLAGS)

# -----------------------------
# Linting, Formatting, Security
# -----------------------------

.PHONY: fmt check lint audit fix

fmt:
	cargo fmt --all

check:
	cargo check

lint:
	cargo clippy --all-targets --all-features -- -D warnings

audit:
	cargo audit

fix:
	cargo clippy --fix --allow-dirty --allow-staged

# -----------------------------
# Distribution
# -----------------------------

.PHONY: dist

dist: build
	@echo "Packing release binary"
	mkdir -p dist
	cp $(TARGET_DIR)/release/$(PROJECT) dist/

