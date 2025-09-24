# Solana SPL Initialized Account (Foundational Store)

This Substreams module tracks SPL token account initializations on Solana, extracting account-to-owner mappings. This is essential for resolving SPL token transfers since the transfer instructions don't contain owner information directly.

If you are a Substreams developer, jump to [Query the Solana SPL Initialized Account Foundation Store](#query-the-solana-spl-initialized-account-foundation-store) for details about how to resolve SPL token ownership when processing SPL Token instructions.

## Query the Solana SPL Initialized Account Foundational Store

This module is typically used by SPL token transfer modules to resolve account ownership. Since SPL transfer instructions only contain account addresses, you need this data to determine who actually sent/received the tokens.

Example usage pattern:
1. Extract SPL transfer instruction with source/destination accounts
2. Query this module's output to get the owner of each account
3. Create transfer records with actual wallet owners instead of just account addresses

> [!NOTE]
> Ensure your Cargo `substreams` and `substreams-solana` dependencies are up to date `cargo update substreams substreams-solana`.

First, add you `substreams.yaml` manifest the SPL Initialized Account import and depend on the Solana SPL Initialized Account Foundation Store:

```yaml
...

imports:
  ...
  spl_initialized_account: spl-initialized-account@v0.1.2

modules:
  ...

  - name: <your-module>
    kind: map
    inputs:
      ...
      - foundational-store: spl-initialized-account@v0.1.2
    output:
      ...
```

Then in your code, adjust your modules Rust input to align with its new definition:

```rust
...
use substreams::store::FoundationalStore;

#[substreams::handlers::map]
fn map_spl_instructions(..., spl_initialized_account_store: FoundationalStore) -> Result<SplInstructions, Error> {
    ...

    // Resolve one owner
    let response = foundational_store.get(<account_address_bytes>);
    if response.response == ResponseCode::Found as i32 {
        if let Ok(account_owner) = AccountOwner::decode(response.value.unwrap().value.as_slice()) {
            substreams::log::info!("Owner is {}", Address(&account_owner.owner).to_string());
        }
    }

    // Resolve batch of owners
    // It's highly recommend to make a single get_all by block to resolve your owner, this will
    // greatly improve the performance of your Substreams
    let response = foundational_store.get_all(<vec of account_address_bytes>);
    for entry in response.entries {
        // Same handling as single case above
    }
}
```

See https://github.com/streamingfast/substreams-spl-token for a full example of SPL Initialized Account Foundational Store usage.

## Data Source

The module processes Solana transactions and extracts information about newly initialized SPL token accounts from these instruction types:
- `InitializeAccount` - Basic account initialization
- `InitializeAccount2` - Account initialization with embedded owner
- `InitializeAccount3` - Account initialization with embedded owner (newer variant)

For each initialized account, it stores the relationship between:
- **Account address** - The token account that was created
- **Mint address** - The SPL token mint this account holds
- **Owner** - The wallet/program that owns this token account

### Configuration

The module is configured to process transactions from both SPL Token programs:
- `TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA` (original)
- `TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb` (Token-2022)
