minimal.wit:
	wasm-tools component wit minimal.wat -o minimal.wit

minimal.wasm: minimal.wit
	wasm-tools parse minimal.wat -o minimal.wasm

run: minimal.wasm
	uv run main.py
