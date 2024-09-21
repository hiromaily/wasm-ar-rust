#!/bin/bash

cargo build

source /emsdk/emsdk_env.sh

# emcc
export EMCC_CFLAGS="-o opencv-wasm.js
                    -s EXPORTED_FUNCTIONS=['_template_match']
                    -s EXPORTED_RUNTIME_METHODS=ccall"

cargo build -p opencv-wasm --target wasm32-unknown-emscripten --release
