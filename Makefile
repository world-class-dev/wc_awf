.PHONY: all build-wasm build-android build-desktop test clean

all: build-wasm build-desktop

build-wasm:
	cargo build --target wasm32-unknown-unknown --release --features wasm

build-android:
	cargo ndk -t arm64-v8a build --release

build-desktop:
	cargo build --release

test:
	cargo test

clean:
	cargo clean
