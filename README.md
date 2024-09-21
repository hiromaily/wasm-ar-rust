# wasm-ar-rust

Wasm AR App

## Install for OpenCV

See [root Makefile](./Makefile)

```sh
brew install opencv
brew install llvm
brew install pkg-config
```

### [opencv-rust](https://github.com/twistedfall/opencv-rust)

- [Install](https://github.com/twistedfall/opencv-rust/blob/master/INSTALL.md)
- [See Makefile](./Makefile)

### configure llvm

```sh
export PATH="/opt/homebrew/opt/llvm/bin:$PATH"
export LDFLAGS="-L/opt/homebrew/opt/llvm/lib"
export CPPFLAGS="-I/opt/homebrew/opt/llvm/include"
export LIBCLANG_PATH=$(brew --prefix llvm)/lib
export DYLD_LIBRARY_PATH="$(xcode-select --print-path)/usr/lib/"
export DYLD_FALLBACK_LIBRARY_PATH="$(xcode-select --print-path)/usr/lib/"
```

## wasmのビルド `Library not loaded: @rpath/libclang.dylib` の対応

```sh
ln -s /Library/Developer/CommandLineTools/usr/lib/libclang.dylib ${HOME}/.local/share/rustup/toolchains/stable-aarch64-apple-darwin/lib/rustlib/aarch64-apple-darwin/lib/libclang.dylib

# unlink
#unlink ${HOME}/.local/share/rustup/toolchains/stable-aarch64-apple-darwin/lib/rustlib/aarch64-apple-darwin/lib/libclang.dylib
```
