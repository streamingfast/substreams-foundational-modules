# Solana Common Modules

The **Solana Common Modules** Substreams contains a set of modules that allow you to easily retrieve basic information from the Solana blockchain, such as transactions or instructions.

The `substreams.yaml` file defines all the different modules available, and also provides you with documentation about the usage of every module.

## Basic Modules

- `blocks_without_votes`: consume a full Solana Block without Vote transactions (i.e. transactions containg `Vote111111111111111111111111111111111111111` instructions). You will have access to the data of a Solana block but the transactions array will not contain Vote transactions.

## Filtered Modules

Usually, you want to only listen for specific transactions or instruction types. The following module allow you to specify a filter based on the program ID of the instruction to efficiently retrive filtered transactions.

- `filtered_transactions_without_votes`: allows you to consume transactions containing instructions from specific program_id based on a filter string.
  Supported operators are: logical or `||`, logical and `&&` and parenthesis: `()`.
  Example: to only consume TRANSACTIONS containing Token or ComputeBudget instructions: `program:TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA || program:ComputeBudget111111111111111111111111111111`.
  Transactions containing `Vote111111111111111111111111111111111111111` are always excluded.