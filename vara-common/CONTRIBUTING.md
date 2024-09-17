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

## New metadata versions

When a new metadata version is added, we need to add it inside of `metadata/spec`.

### Fetch the new metadata version and save it locally

Run the latest `firevara` binary to fetch and save the new version json locally. Then copy that file to `metadata/references/`.

```bash
# there is a specVersions.txt file found at /cmd/firevara/tools-data/specVersions.txt (inside of firehose-gear repository)
firevara tools-fetch-metadata --spec-version-path /path/to/specVersions.txt --versions <new-version>
cp <new-version>.json metadata/references/metadata<version>.json
```

### Fetch the hex of the metadata

Run the below curl command to fetch the hex value of the new version metadata. It is the hex value of the JSON of the metadata.

```bash
curl --location 'https://vara-mainnet.public.blastapi.io' --header 'Content-Type: application/json' --data '{ "id": 1, "jsonrpc": "2.0", "method": "state_getMetadata", "params": ["<blockhash_where_version_changed>"] }' | jq .result > <version>.hex
```

Then add a new file inside of the `metadata/spec` names <version>.go.

```golang
// filename: 1501.go
package spec

var V1501 = "contents of 1501.hex"
```

Then you can safely delete the `<version>.hex` file.

### Add new tests for new metadata files

```bash
# <block> here, pick the first boundary block of the specVersion block hash
firecore tools print merged-blocks "path/to/merged-blocks" <block> -o jsonl --bytes-encoding=base64 > testdata/<block>.blocks.jsonl

# <block> here, pick one much later after the new spec version
firecore tools print merged-blocks "path/to/merged-blocks" <block> -o jsonl --bytes-encoding=base64 > testdata/<block>.blocks.jsonl

# validate that the version is the same for all the 100 blocks, if not take the next boundary
cat vara-common/testdata/<version>.blocks.jsonl | grep "spec_version\":<version>" | wc -l
```
