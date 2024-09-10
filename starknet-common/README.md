# Starknet Foundational Substreams in Go with tinygo

First test is to receive a Clock in Go, and ship it to the Substreams engine, and have it run over there.

- Craft a first `map_mod` that prints something to logs.

First test is to unpack a raw Ethereum Block, from within `tinygo`.

## Build

```bash
tinygo build -o wasm.wasm -target wasi -gc leaking -scheduler none .
```

## Usage

### Stream  
```bash
substreams gui ./substreams-starknet.yaml --plaintext -e 127.0.0.1:10016 -t +10 map_test
# or
substreams run ./substreams-starknet.yaml --plaintext -e 127.0.0.1:10016 -t +10 map_test
```

### Stream specific data
To stream data only, from a specific contract (`0x001f4466085c4bb3374ecad67bfcb4cce25ea502617ab624cf532f90300f2794`) or to get a specific transaction... You'll need to use parameters.

```bash
substreams run ./substreams-starknet.yaml --plaintext -t +10 filtered_transactions -p filtered_transactions=ev:from_address:0x1f4466085c4bb3374ecad67bfcb4cce25ea502617ab624cf532f90300f2794
```

> [!NOTE]
> Starknet explorers ([StarknetScan](https://starkscan.co/) for example) might pad with leading zeros, addresses or hashes. Make sure, using parameters, to remove those leading zeros, otherwise
> the index module key won't match with the parameter provided...


```bash
buf generate --exclude-path sf/substreams-starknet/v1,sf/substreams-starknet/rpc,google/,sf/substreams-starknet/sink,sf/substreams-starknet
```
