.PHONY: all build serve
all: build serve

build:
	cargo build --target wasm32-unknown-unknown
	wasm-bindgen --out-dir target --target web target/wasm32-unknown-unknown/debug/minimal_webgl2_crash.wasm
	#wasm-opt --debuginfo -Oz target/minimal_webgl2_crash_bg.wasm -o target/minimal_webgl2_crash_bg_opt.wasm
	#sed -i 's/_bg\.wasm/_bg_opt\.wasm/g' target/minimal_webgl2_crash.js

serve:
	python3 -m http.server
