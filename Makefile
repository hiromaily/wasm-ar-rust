#------------------------------------------------------------------------------
# OpenCV
#------------------------------------------------------------------------------

.PHONY: pre-install
pre-install:
	brew install opencv
	brew install pkg-config

# required for building wasm
# .PHONY: setup-libclang
# setup-libclang-wasm:
# 	@#ln -s /opt/homebrew/opt/llvm/lib/libclang.dylib ${HOME}/.local/share/rustup/toolchains/stable-aarch64-apple-darwin/lib/rustlib/aarch64-apple-darwin/lib/libclang.dylib
# 	ln -s /Library/Developer/CommandLineTools/usr/lib/libclang.dylib ${HOME}/.local/share/rustup/toolchains/stable-aarch64-apple-darwin/lib/rustlib/aarch64-apple-darwin/lib/libclang.dylib
# 	@#unlink ${HOME}/.local/share/rustup/toolchains/stable-aarch64-apple-darwin/lib/rustlib/aarch64-apple-darwin/lib/libclang.dylib


#------------------------------------------------------------------------------
# Wasm
#------------------------------------------------------------------------------
.PHONY: install-wasm-pack
install-wasm-pack:
	cargo install wasm-pack

.PHONY: lint
lint:
	cargo fmt --all
	cargo clippy --all-targets --all-features

# before run
# `cargo install cargo-machete` is required
.PHONY: check-deps
check-deps:
	cargo machete

.PHONY: fix
fix:
	cargo fix --allow-staged

.PHONY: build
build:
	cargo build
	#make -C crates/opencv-wasm build

.PHONY: setup-web
setup-web:
	cd wasm-frontend; npm install

.PHONY: run-web
run-web:
	make -C wasm-frontend dev


#------------------------------------------------------------------------------
# Docker
#------------------------------------------------------------------------------
.PHONY: build-image
build-image:
	docker build --progress=plain -t wasm-ar:latest -f ./Dockerfile .

.PHONY: lint-dockerfile
lint-dockerfile:
	docker build -f ./Dockerfile . --check
