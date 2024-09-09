# Contributing

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
