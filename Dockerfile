FROM rust:1.81.0-bullseye

RUN apt-get update && apt-get install -y \
    clang \
    cmake \
    curl \
    g++ \
    libopencv-dev \
    ninja-build \
    pkg-config \
    && apt-get clean

# install wasm-pack
RUN curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

# install Emscripten
RUN git clone https://github.com/emscripten-core/emsdk.git \
    && cd emsdk \
    && ./emsdk install latest \
    && ./emsdk activate latest

RUN rustup target add wasm32-unknown-emscripten

WORKDIR /workspace

# copy code # Note: make sure .dockerignore is configured properly
COPY . .

# build multiple workspaces
#RUN make build
RUN ./build.sh

CMD ["bash"]
