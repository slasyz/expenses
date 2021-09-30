.PHONY: frontend-watch
frontend-watch:
	cargo watch -i .gitignore -i "frontend/pkg/*" -s "make frontend-build"

.PHONY: frontend-build
frontend-build:
	wasm-pack build --target web ./frontend
	rollup ./resources/main.js --format iife --file ./frontend/pkg/bundle.js

.PHONY: frontend-serve
frontend-serve:
	python3 -m http.server 8000
