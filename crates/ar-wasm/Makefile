# can be used pkg from npm
.PHONY: build
build:
	mkdir -p public/pkg
	wasm-pack build --out-dir public/pkg-bundler --target bundler

.PHONY: build-web
build-web:
	mkdir -p public/pkg
	wasm-pack build --out-dir public/pkg --target web

