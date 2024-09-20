# llvm
# export PATH="/opt/homebrew/opt/llvm/bin:$PATH"
# export LDFLAGS="-L/opt/homebrew/opt/llvm/lib"
# export CPPFLAGS="-I/opt/homebrew/opt/llvm/include"
# export LIBCLANG_PATH=$(brew --prefix llvm)/lib
# export DYLD_LIBRARY_PATH="$(xcode-select --print-path)/usr/lib/"
# export DYLD_FALLBACK_LIBRARY_PATH="$(xcode-select --print-path)/usr/lib/"
.PHONY: pre-install
pre-install:
	brew install opencv
	brew install llvm
	brew install pkg-config
	ln -s /opt/homebrew/opt/llvm/lib/libclang.dylib ${HOME}/.local/share/rustup/toolchains/stable-aarch64-apple-darwin/lib/rustlib/aarch64-apple-darwin/lib/libclang.dylib

.PHONY: build
build:
	mkdir -p public/pkg
	wasm-pack build --out-dir public/pkg --target web

# can be used pkg from npm
.PHONY: build-bundler
build-bundler:
	mkdir -p public/pkg
	wasm-pack build --out-dir public/pkg-bundler --target bundler
