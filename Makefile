DEFAULT_GOAL := run

minimal.wit: minimal.wat
	wasm-tools component wit minimal.wat -o minimal.wit

minimal.wasm: minimal.wit
	wasm-tools parse minimal.wat -o minimal.wasm

.PHONY: build
build: minimal.wasm

.PHONY: run
run: build
	uv run main.py

.PHONY: clean
clean:
	rm -f minimal.wasm minimal.wit
