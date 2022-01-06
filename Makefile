wasm:
	wasm-pack build --target web
	rollup ./main.js --format iife --file ./pkg/bundle.js
	cp -a ./external ./pkg/external
	rm ./pkg/.gitignore
