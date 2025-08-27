# Solana SPL Owner Foundational Store

This foundational module provides the `AccountOwner` message for Solana SPL token account ownership data.

## Message Definition

```protobuf
message AccountOwner {
  bytes mint = 1;
  bytes owner = 2;
}
```

## Usage

### Importing in substreams.yaml

After building this module with `substreams build`, you can import it in your consumer substreams:

```yaml
imports:
  spl_owner_foundational: https://spkg.io/streamingfast/solana-spl-owner-foundational-store-v0.1.0.spkg

protobuf:
  files:
    - your/proto/files.proto
  importPaths:
    - ./proto

modules:
  - name: your_module
    kind: map
    inputs:
      - source: sf.solana.type.v1.Block
    output:
      type: proto:your.package.version.YourMessage
```

### Available Protobuf Types

After running `substreams build`, the following Protobuf type will be available:
- `sf.solana.spl.foundational.v1.AccountOwner`

### Rust Usage Example

When using this module in another substreams project that imports it, use:

```rust
use pb::sf::solana::spl::foundational::v1::AccountOwner;

// Create an AccountOwner message
let account_owner = AccountOwner {
    mint: mint_pubkey.to_bytes().to_vec(),
    owner: owner_pubkey.to_bytes().to_vec(),
};
```

This module can be imported and used by other substreams that need to work with account ownership data in the Solana ecosystem.
