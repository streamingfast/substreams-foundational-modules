.PHONY: build
build:
	tinygo build -o wasm.wasm -target wasi -gc leaking -scheduler none .

.PHONY: run
run:
	substreams run ./substreams.yaml --plaintext -e 127.0.0.1:10016 -t +10 map_decoded_block
