.PHONY: build
build:
	tinygo build -o wasm.wasm -target wasi -gc leaking -scheduler none . && wizer wasm.wasm  --allow-wasi -o my2-wasm.wasm -f _start  --wasm-bulk-memory true 

.PHONY: run
run:
	substreams run ./substreams.yaml --plaintext -e 127.0.0.1:10016 -t +10 map_decoded_block
