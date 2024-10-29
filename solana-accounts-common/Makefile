.PHONY: build

protogen:
	 substreams protogen ./substreams.yaml --exclude-paths="google"

build:
	tinygo build -o tiny.wasm -target wasi -gc leaking -scheduler none . &&  wizer tiny.wasm  --allow-wasi -o wasm.wasm -f _start  --wasm-bulk-memory true

