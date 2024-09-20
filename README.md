# wasm-ar-rust

Wasm AR App

## ar-wasm

### [opencv-rust](https://github.com/twistedfall/opencv-rust)

- [Install](https://github.com/twistedfall/opencv-rust/blob/master/INSTALL.md)
- [See Makefile](./Makefile)

### llvm

```sh
export PATH="/opt/homebrew/opt/llvm/bin:$PATH"
export LDFLAGS="-L/opt/homebrew/opt/llvm/lib"
export CPPFLAGS="-I/opt/homebrew/opt/llvm/include"
export LIBCLANG_PATH=$(brew --prefix llvm)/lib
export DYLD_LIBRARY_PATH="$(xcode-select --print-path)/usr/lib/"
export DYLD_FALLBACK_LIBRARY_PATH="$(xcode-select --print-path)/usr/lib/"
```
