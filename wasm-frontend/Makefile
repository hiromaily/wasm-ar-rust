.PHONY: dev
dev:
	npm run dev

.PHONY: lint
lint:
	npx @biomejs/biome check --write ./src


.PHONY: install-wasm-pkg
install-wasm-pkg:
	npm i ../crates/opencv-wasm/public/pkg-bundler
