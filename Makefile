.PHONY: frontend-watch
frontend-watch:
	cargo watch -i .gitignore -i "pkg/*" -s "make frontend-build"

.PHONY: frontend-build
frontend-build:
	wasm-pack build --target web
	rollup ./main.js --format iife --file ./pkg/bundle.js

.PHONY: frontend-serve
frontend-serve:
	python3 -m http.server 8000
