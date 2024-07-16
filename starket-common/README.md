# Substreams in Go with tinygo

First test is to receive a Clock in Go, and ship it to the Substreams engine, and have it run over there.

- Craft a first `map_mod` that prints something to logs.

First test is to unpack a raw Ethereum Block, from within `tinygo`.

## Build

```bash
tinygo build -o wasm.wasm -target wasi -gc leaking -scheduler none .
```

## Usage

```bash
substreams gui ./substreams-starknet.yaml --plaintext -e 127.0.0.1:10016 -t +10 map_test
# or
substreams run ./substreams-starknet.yaml --plaintext -e 127.0.0.1:10016 -t +10 map_test
```

```bash
buf generate --exclude-path sf/substreams-starknet/v1,sf/substreams-starknet/rpc,google/,sf/substreams-starknet/sink,sf/substreams-starknet
```
