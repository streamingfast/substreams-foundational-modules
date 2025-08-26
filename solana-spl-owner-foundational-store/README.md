# Solana SPL Owner Foundational Store

This foundational module provides the `AccountOwner` message for Solana SPL token account ownership data.

## Message Definition

```protobuf
message AccountOwner {
  bytes mint = 1;
  bytes owner = 2;
}
```

This module can be imported and used by other substreams that need to work with account ownership data in the Solana ecosystem.