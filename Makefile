SHELL := /bin/bash

.PHONY: build build-watch test test-watch server open setup

build:
	wasm-pack build --target no-modules --out-dir target --out-name index
	wasm-opt -O3 -o target/index.wasm target/index_bg.wasm

build-watch:
	cargo watch -s 'make build'

test:
	cargo test

test-watch:
	cargo watch -s 'make test'

server:
	python -m SimpleHTTPServer

open:
	open http://localhost:8000

setup:
	rustup target add wasm32-unknown-unknown
	which wasm-pack || cargo install wasm-pack
	which cargo-watch || cargo install cargo-watch
	brew install binaryen
