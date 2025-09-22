# SPL Initialized Account

This Substreams module tracks SPL token account initializations on Solana, extracting account-to-owner mappings. This is essential for resolving SPL token transfers since the transfer instructions don't contain owner information directly.

## What it does

The module processes Solana transactions and extracts information about newly initialized SPL token accounts from these instruction types:
- `InitializeAccount` - Basic account initialization
- `InitializeAccount2` - Account initialization with embedded owner
- `InitializeAccount3` - Account initialization with embedded owner (newer variant)

For each initialized account, it stores the relationship between:
- **Account address** - The token account that was created
- **Mint address** - The SPL token mint this account holds
- **Owner** - The wallet/program that owns this token account

## Output Format

The module outputs `sf.substreams.foundational_store.v1.Entries` where each entry contains:
- **Key**: Token account address (as bytes)
- **Value**: `AccountOwner` protobuf message containing mint address and owner (both as bytes)

## Usage in other Substreams

This module is typically used by SPL token transfer modules to resolve account ownership. Since SPL transfer instructions only contain account addresses, you need this data to determine who actually sent/received the tokens.

Example usage pattern:
1. Extract SPL transfer instruction with source/destination accounts
2. Query this module's output to get the owner of each account
3. Create transfer records with actual wallet owners instead of just account addresses

## Configuration

The module is configured to process transactions from both SPL Token programs:
- `TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA` (original)
- `TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb` (Token-2022)
