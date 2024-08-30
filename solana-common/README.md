# Solana Common Modules

The **Solana Common Modules** Substreams contains a set of modules that allow you to easily retrieve basic information from the Solana blockchain, such as transactions or instructions.

The `substreams.yaml` file defines all the different modules available, and also provides you with documentation about the usage of every module.

## Modules

### `blocks_without_votes`

* This module is a mapper that provides drop-in replacement of the raw solana blocks, but uses 25% of its storage by pruning all the "Vote Transactions" (i.e. transactions containg `Vote111111111111111111111111111111111111111` instructions) from the 'transactions' array.

### `transactions_by_programid_and_account_without_votes`

* This module allows you to consume transactions containing instructions from specific program_id or impacting an account, based on a filter string.

* Supported operators are: logical or `||`, logical and `&&` and parenthesis: `()`.

* Example: to only consume TRANSACTIONS containing Token or ComputeBudget instructions touching the address : `program:TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA || program:ComputeBudget111111111111111111111111111111`.
Transactions containing `Vote111111111111111111111111111111111111111` are always excluded.

### `program_ids_and_accounts_without_votes`

* This module is an index that allows you to efficiently retrieve instructions based on the program ID.
      For example, the following sets keys for the Token program:
      * program:TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA
      * account:someOtherAccountEncodedInBase54111111111111



## Filtered Modules

Usually, you want to only listen for specific transactions or instruction types. The following module allow you to specify a filter based on the program ID of the instruction to efficiently retrive filtered transactions.

- `filtered_transactions_without_votes`: allows you to consume transactions containing instructions from specific program_id based on a filter string.
  Supported operators are: logical or `||`, logical and `&&` and parenthesis: `()`.
  Example: to only consume TRANSACTIONS containing Token or ComputeBudget instructions: `program:TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA || program:ComputeBudget111111111111111111111111111111`.
  Transactions containing `Vote111111111111111111111111111111111111111` are always excluded.