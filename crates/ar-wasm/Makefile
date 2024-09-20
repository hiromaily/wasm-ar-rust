
.PHONY: build
build:
	mkdir -p public/pkg
	wasm-pack build --out-dir public/pkg --target web

# can be used pkg from npm
.PHONY: build-bundler
build-bundler:
	mkdir -p public/pkg
	wasm-pack build --out-dir public/pkg-bundler --target bundler
