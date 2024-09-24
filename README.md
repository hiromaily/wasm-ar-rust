# wasm-ar-rust

Wasm AR App

## What do I want to learn through this project?

- Learn WASM using Rust
- Image detection
- Image manipulation
- WebGPU API

## Image detection

### [opencv-rust](https://github.com/twistedfall/opencv-rust) is not used

This library depends on C/C++ library. It means it's difficult to compile by `wasm-pack build --target bundler`. Even if giving up `wasm-pack`, [emscripten-core/emsdk](https://github.com/emscripten-core/emsdk) is a bit tricky.

### Image detection algorithm

#### [Template Matching](https://en.wikipedia.org/wiki/Template_matching) without OpenCV

[imageproc](https://crates.io/crates/imageproc) crate has implementation. However it was extreamly slow.  
Then I found `WebGPU` based [template matching](https://crates.io/crates/template-matching) library. It is quite good performance, but code is messy and poor handling of asynchronous processing and errors which cause panic on WASM environment. In the end I forked and fixed theses issue.

#### using [pHash](https://www.phash.org/)

This is hashing algorithm for identifying identical images. I tried but it was slow. At this moment I skipped to use it, but I need to dig in more deeply to understand more.

## Image manipulation

As with `Image detection` above, opencv is not used. Instead of it, [image](https://crates.io/crates/image) and [imageproc](https://crates.io/crates/imageproc) crates are used.

## WIP: Drawing image from WASM

First, I drew images generated from wasm into canvas area in frontend.

### Hnadling 3D data on WASM

## Docs

[docs](./docs/README.md)
