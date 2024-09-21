# wasm-ar-rust

Wasm AR App

## Install for OpenCV

See [root Makefile](./Makefile)

```sh
brew install opencv
brew install pkg-config
```

### [opencv-rust](https://github.com/twistedfall/opencv-rust)

- [Install](https://github.com/twistedfall/opencv-rust/blob/master/INSTALL.md)
- [See Makefile](./Makefile)

```sh
brew install opencv

export DYLD_FALLBACK_LIBRARY_PATH="$(xcode-select --print-path)/Toolchains/XcodeDefault.xctoolchain/usr/lib/"
export LDFLAGS=-L/Library/Developer/CommandLineTools/SDKs/MacOSX.sdk/usr/lib
export LD_LIBRARY_PATH=${LD_LIBRARY_PATH}:/usr/local/lib
```
