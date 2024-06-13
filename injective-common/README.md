# Injective Foundational Modules

The Injective Foundational modules are Substreams modules extracting common data from the Injective blockchain.

## Modules

### all_events

The `all_events` module extracts only the events and provides them, along with the transaction hash and block at which it they are found.

### filtered_events

The `filtered_events` module allows a reduction of the `all_events` output, only matching the events that match the requested type.

Use with the parameter, ex:

```bash
substreams run [...] -p filtered_events="message || injective.peggy.v1.EventDepositClaim"
```

### all_transactions (work in progress)

The `all_transactions` module extracts all the transactions from the Injective blockchain, providing useful information, such as _messages_, _signatures_ or _events_.

Some message types are parsed from their "Any" type into the the corresponding type of an enum. See ./proto/cosmos/v1/transactions.proto to see the ones that are supported.
The other types will still be shown as protobuf "Any" type.

### filtered_trx_by_events

The `filtered_trx_by_events` modules allows a reduction of the `all_transactions` output, only matching the events that match the requested type. The module will return the entire transactions. Some event types will appear that do not match from the filtered params as the entire transaction is returned.

Use with the parameter, ex:

```bash
substreams run [...] -p filtered_trx_by_events="message || injective.peggy.v1.EventDepositClaim"
```

## Getting Started

### Gather protobuf definitions in generated-buf-build.binpb

The required protobuf modules are referenced in `buf.yaml`.
You need a (free) API token to access https://buf.build and resolve the dependencies into a single file, generated-buf-build.binpb.
That file is then used to generate the rust protobuf bindings or to bundle the definitions in the .spkg. (it is referenced in the substreams.yaml)

### Generate rust protobuf bindings

```bash
make protogen
```

### Build Substreams modules

```bash
make build
```

### Run Substreams

You will need an API key to access the streamingfast servers, see https://substreams.streamingfast.io

This example query only fetches the events of type 'injective.peggy.v1.EventDepositClaim'

```bash
substreams run -e mainnet.injective.streamingfast.io:443 substreams.yaml filtered_events -p filtered_events='injective.peggy.v1.EventDepositClaim' -s 9600 -t 9700
```

### Package as .spkg

```bash
make package
```
