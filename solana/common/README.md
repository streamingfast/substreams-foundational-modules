# Solana Common Modules

The **Solana Common Modules** Substreams contains a set of modules that allow you to easily retrieve basic information from the Solana blockchain, such as transactions or instructions.

The `substreams-v0.3.0.yaml` file defines all the different modules available, and also provides you with documentation about the usage of every module.

# Using this module to speed up a substreams

## Using the full "solana block" object (simplest if changing an existing substreams)

In your substreams.yaml,

1. Import this .spkg:

```
imports:
  sol: https://spkg.io/streamingfast/solana-common-v0.3.0.spkg
```

2. Replace any `source: sf.solana.type.v1.Block` input with `map: sol:blocks_without_votes` (you will be getting the same protobuf object, but with some vote-related transactions already pruned)

3. Add block filtering to your "entry modules" (any module reading blocks or transactions before emitting your custom types):

If you know the instruction `program ID` of all transactions that you want, use the `program_ids_without_votes` index like this:

```
    blockFilter:
      module: sol:program_ids_without_votes
      query:
        string: "program:4vMsoUT2BWatFweudnQM1xedRLfJgJ7hswhcpz4xgBTy || program:Gnt27xtC473ZT2Mw5u8wZ68Z3gULkSTb5DuxJy7eJotD"
```

If you also need to get transactions based on the instruction's `accounts`, use the (bigger) index like this:

```
    blockFilter:
      module: sol:program_ids_and_accounts_without_votes
      query:
        string: "program:4vMsoUT2BWatFweudnQM1xedRLfJgJ7hswhcpz4xgBTy || account:Gnt27xtC473ZT2Mw5u8wZ68Z3gULkSTb5DuxJy7eJotD"
```

## Using the new 'transactions' object (simplest if writing a new substreams)

In your substreams.yaml,

1. Import this .spkg:

```
imports:
  sol: https://spkg.io/streamingfast/solana-common-v0.3.0.spkg
```

2. Set one of `transactions_by_programid_without_votes` or `transactions_by_programid_and_account_without_votes` (along with `source: sf.substreams.v1.Clock` if you need slot number/ID/timestamp) as your module input, ex:

```
  - name: my_cool_module
    kind: map
    inputs:
      - source: sf.substreams.v1.Clock
      - map: sol:transactions_by_programid_without_votes
```

3. Set the "block filter string" on the `sol:transactions_by_programid_without_votes` or `transactions_by_programid_and_account_without_votes` module to match the data that you want to be fed to your module:

```
params:
  sol:transactions_by_programid_without_votes: "program:4vMsoUT2BWatFweudnQM1xedRLfJgJ7hswhcpz4xgBTy"
  sol:transactions_by_programid_and_account_without_votes: "program:4vMsoUT2BWatFweudnQM1xedRLfJgJ7hswhcpz4xgBTy || (account:Gnt27xtC473ZT2Mw5u8wZ68Z3gULkSTb5DuxJy7eJotD && program:FsJ3A3u2vn5cTVofAjvy6y5kwABJAqYWpe4975bi2epH)"
```

4. Run `substreams protogen` against your substreams.yaml to create the rust bindings of the protobuf definition inside the substreams. (If you are using the `substreams-solana` rust crate, follow the instructions to write a `buf.gen.yaml` that will bind things correctly: https://github.com/streamingfast/substreams-solana, ) 

## Modules

### `blocks_without_votes` (map)

* This module is a mapper that provides drop-in replacement of the raw solana blocks, but uses 25% of its storage by pruning all the "Vote Transactions" (i.e. transactions containg `Vote111111111111111111111111111111111111111` instructions) from the 'transactions' array.

### `program_ids_without_votes` (index)

* This module is an index that allows you to efficiently retrieve instructions based on the program ID.
  Example keys: `program:TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA`, `program:4vMsoUT2BWatFweudnQM1xedRLfJgJ7hswhcpz4xgBTy`

Use it to only get blocks that contain instructions matching the programID/accounts that you need, ex:

```
  - name: my_module
    ...
    blockFilter:
      module: program_ids_without_votes
      query:
        string: (program:4vMsoUT2BWatFweudnQM1xedRLfJgJ7hswhcpz4xgBTy || program:TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA)
```

### `program_ids_and_accounts_without_votes` (index)

* This module is an index that allows you to efficiently retrieve instructions based on the program ID and/or accounts.
   Example keys: `account:Gnt27xtC473ZT2Mw5u8wZ68Z3gULkSTb5DuxJy7eJotD`, `program:TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA`  

Use it to only get blocks that contain instructions matching the programID/accounts that you need, ex:

```
  - name: my_module
    ...
    blockFilter:
      module: program_ids_and_accounts_without_votes
      query:
        string: (account:Gnt27xtC473ZT2Mw5u8wZ68Z3gULkSTb5DuxJy7eJotD && program:TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA)
```

### `transactions_by_programid_without_votes` (map)

* This modules uses the `program_ids_without_votes` index to only process blocks that contain matching instructions. 
* Furthermore, it filters out all the transactions that do not contain a matching instruction, so your module can be run only on data that is meaningful to you.
* Set the query string by using params (ref: https://substreams.streamingfast.io/documentation/develop/parameterized-modules)

### `transactions_by_programid_and_account_without_votes` (map)

* This modules uses the `program_ids_and_accounts_without_votes` index to only process blocks that contain matching instructions. 
* Furthermore, it filters out all the transactions that do not contain a matching instruction, so your module can be run only on data that is meaningful to you.
* Set the query string by using params (ref: https://substreams.streamingfast.io/documentation/develop/parameterized-modules)
