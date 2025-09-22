# Solana Accounts Foundational module

This package allows filtered access to a stream of account changes based on the account or the owner of that account.

## Example query parameter

* Only accounts owned by Tokenkeg: `owner:TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA`
* Change to a list of accounts: `account:AwjomjhbNqkgEZN1ADvbEcTYsDfBuX4AKzNmxwgtKvxM || account:5Q544fKrFoe6tsEbD7S8EmxGTJYAKtTVhAW5Q5pge4j1`

## Example response:

note: the response is in bytes, we represented the address and owner in base54 and the data in hex for the sake of clarity.

```
[
      {
        "address": "HxGjvBoVFNX2gXzsJBA5Ttd61WH8RV8YXyA7yaVF8rnE",
        "owner": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA",
        "data": "069b8857feab8184fb687f634618c035dac439dc1aeb3b5598a0f000000000018d91334d84916c39"
      },
      {
        "address": "J4kiQKJXCQm4Khmw5mtKvjfW7noFiF48Ngvoekp64Jdy",
        "owner": "00000000000000000000000000000000000000000000000",
        "data": "",
        "deleted": true
      }
]
```

* The slot number, hash and timestamp are not in the response object, as they are part of the Clock returned as part of any Substreams response..

## Restrictions

Note: These restrictions are on the Substreams wsolana-accounts" endpoint itself, not implemented in this package. They are listed for your convenience.

* An individual account's changes are "rounded-up" to each block (for multiple changes to the same account within a block, you get a single event with the last value)
* Changes that only affect the lamports are omitted (unless they cause the deletion of the account)
* Accounts changes under the `Vote111111111111111111111111111111111111111` owner are all filtered out directly on the Substreams endpoint: they cannot be queried here.
