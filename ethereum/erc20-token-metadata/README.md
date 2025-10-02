# Token Metadata Foundational Store

A Substreams module that extracts token metadata (name, symbol, decimals) from EVM blockchains and outputs it in a format compatible with the [Substreams Foundational Store](https://github.com/streamingfast/substreams-foundational-store/tree/develop).

## Overview

This project was created for use with the **Substreams Foundational Store** from StreamingFast, which provides a high-performance, multi-backend key-value storage system designed for Substreams data ingestion and serving.

The substream imports the [ERC20 Metadata substream created by Pinax](https://github.com/pinax-network/substreams-evm-tokens) for the [TokenAPI](https://token-api.thegraph.com/), extending it to output data in the Foundational Store format.

## Features

- **ERC20 Token Support**: Currently supports ERC20 tokens with plans to extend to ERC721, ERC1155, and other token standards
- **Foundational Store Compatible**: Outputs `Entries` with key-value pairs where values are `google.protobuf.Any` messages
- **RPC Integration**: Fetches complete token metadata via batch RPC calls for metadata change events
- **Efficient Processing**: Handles both initialization events (direct extraction) and change events (RPC lookup)

## Data Model

The substream outputs token metadata using the following protobuf structure:

```protobuf
syntax = "proto3";

package evm.token.metadata.v1;

message TokenMetadata {
  bytes  address  = 1;
  string name     = 2;
  string symbol   = 3;
  int32  decimals = 4;
}
```

Each `TokenMetadata` message is wrapped in a `google.protobuf.Any` and stored as the value in a Foundational Store `Entry`, with the token address as the key.

## Architecture

### Input Processing

The substream processes two types of events:

1. **MetadataInitialize Events**: Token metadata is extracted directly from the event data
2. **MetadataChanges Events**: Complete metadata is fetched via batch RPC calls to ensure accuracy

### Output Format

The module outputs `foundational_store.v1.Entries` containing:
- **Key**: Token contract address (bytes)
- **Value**: `TokenMetadata` message wrapped in `google.protobuf.Any`

This format is optimized for ingestion by the Foundational Store sink.

## Usage

### Prerequisites

- Rust toolchain with `wasm32-unknown-unknown` target
- Substreams CLI
- API key from [thegraph.market](thegraph.market)

### Building

```bash
# Install WASM target if not already installed
rustup target add wasm32-unknown-unknown

# Generate protobuf bindings
make protogen

# Build the WASM module
cargo build --release --target wasm32-unknown-unknown
```

### Running

```bash
# Run the substream
substreams run substreams.yaml metadata_to_foundational_store -e <your-endpoint>

# Package for deployment
substreams pack substreams.yaml
```

## Roadmap

### Current Support
- ✅ ERC20 tokens (name, symbol, decimals)

### Planned Extensions
- 🔄 ERC721 NFT metadata support
- 🔄 ERC1155 multi-token support  
- 🔄 Additional token standards
- 🔄 Enhanced metadata fields (URI, description, etc.)

## Development

### Project Structure

```
├── proto/
│   ├── foundational-store.proto    # Foundational Store definitions
│   └── token-metadata.proto        # Token metadata structure
├── src/
│   ├── lib.rs                      # Main substream logic
│   ├── rpc.rs                      # RPC batch utilities
│   └── pb/                         # Generated protobuf bindings
├── substreams.yaml                 # Substream manifest
└── Cargo.toml                      # Rust dependencies
```

### Key Dependencies

- `substreams`: Core substreams functionality
- `prost`: Protobuf encoding/decoding
- `prost-types`: Well-known protobuf types (`Any`)
- `substreams-ethereum`: Ethereum RPC utilities

## License

This project follows the same license as the foundational components it builds upon.

## Acknowledgments

- **StreamingFast**: For the [Substreams Foundational Store](https://github.com/streamingfast/substreams-foundational-store/tree/develop)
- **Pinax Network**: For the [ERC20 Metadata substream](https://github.com/pinax-network/substreams-evm-tokens) foundation

## Related Projects

- [Substreams Foundational Store](https://github.com/streamingfast/substreams-foundational-store) - High-performance storage backend
- [Pinax EVM Tokens](https://github.com/pinax-network/substreams-evm-tokens) - Base ERC20 metadata extraction
- [Substreams](https://substreams.streamingfast.io/) - Real-time blockchain data processing platform