# Vara Mainnet Foundational Modules

In this repository, you will find all the foundational modules containing various mappers and indexed Substreams. These foundational modules allow users to easily consume any `Extrinsic` or `Even ` from the Vara Mainnet blockchain. The Substreams were written in Golang using `tinygo`, for [reference](https://tinygo.org/) and `wizer`, for [reference](https://github.com/bytecodealliance/wizer?tab=readme-ov-file#install)

## Install tinygo

Follow the instructions to install `tinygo` [here](https://tinygo.org/getting-started/install/).

## Build

First step is to build the Substreams which will produce a wasm binary.

```bash
make build
```

## Proto types generation

Navigate to the proto directory and generate all the protobuf types using [buf](https://buf.build/).

```bash
cd proto
buf generate  --include-imports
```

## Usage

### Augmented Block

The raw `sf.gear.type.v1.Block` will contain the `Extrinsics` and the `Events` of each block in raw format. Run `map_decoded_block` to get the augmented block with all `Extrinsics` and `Events` decoded.

```bash
substreams run ./substreams.yaml -e mainnet.vara.streamingfast.io:443 -t +1 map_decoded_block
```

### All Extrinsics

The `all_extrinsics` module will output the extrinsics decoded with this format `sf.substreams.gear.type.v1.Extrinsics`.

```bash
substreams run ./substreams.yaml -e mainnet.vara.streamingfast.io:443 -t +1 all_extrinsics
```

### Filtered Extrinsics

This module is probably the most interesting one to use. The `filetered_extrinsics` module will output only the extrinsics of the query passed in `--params`.

```bash
substreams run ./substreams.yaml -e mainnet.vara.streamingfast.io:443 -t +1 filtered_extrinsics --params='extrinsic:Timestamp.set:event:System.ExtrinsicSuccess'
```
