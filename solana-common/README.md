# Solana Common Modules

The **Solana Common Modules** Substreams contains a set of modules that allow you to easily retrieve basic information from the Solana blockchain, such as transactions or instructions.

The `substreams.yaml` file defines all the different modules available, and also provides you with documentation about the usage of every module.

## Basic Modules

- `map_block_without_votes`: consume a full Solana Block without Vote transactions (i.e. transactions containg `Vote111111111111111111111111111111111111111` instructions). You will have access to the data of a Solana block but the transactions array will not contain Vote transactions.
- `all_transactions_without_votes`: consume all of the transactions of a block directly, excluding the Vote transactions.
- `all_instructions_without_votes`: consume all the intructions of a block directly, excluding the Vote transactions.

## Filtered Modules

Usually, you want to only listen for specific transactions or instruction types. The following module allow you to specify a filter based on the program ID of the instruction to efficiently retrive filtered transactions and instructions.

- `filtered_instructions_without_votes`: use logical operators (`&&`, `||`) to select from which specific instructions you want to consume data. For example: `program:TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA || program:ComputeBudget111111111111111111111111111111` will only retrieve Token and ComputeBudget instructions.
- `filtered_txs_by_instructions_without_votes`: consume all the transactions containing a specific instruction. You will get access to the full transaction data, but you will use the logical operators (`&&`, `||`) to select which specific instructions you want to the transaction to contain. For example, `program:TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA || program:ComputeBudget111111111111111111111111111111` will get all the transactions that contain the Token and ComputeBudget instructions.