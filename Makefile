SHELL := /bin/bash

.PHONY: build build-watch test test-watch server open setup

build:
	wasm-pack build --target no-modules --out-dir target --out-name index
	wasm-opt -O3 -o target/index.wasm target/index_bg.wasm

build-dev:
	wasm-pack build --dev --target no-modules --out-dir target --out-name index
	mv target/index_bg.wasm target/index.wasm

build-watch:
	cargo watch -s 'clear && make build-dev' --delay 0

test:
	cargo test

test-watch:
	cargo watch -s 'clear && make test' --delay 0

server:
	live-server --watch=target/index.wasm

setup:
	rustup target add wasm32-unknown-unknown
	which wasm-pack || cargo install wasm-pack
	which cargo-watch || cargo install cargo-watch
	brew install binaryen
	yarn global add live-server
