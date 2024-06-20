# Injective Foundational Modules

The Injective Foundational modules are Substreams modules extracting common data from the Injective blockchain.

## Getting the substreams package

### From Substreams.dev

https://substreams.dev/streamingfast/injective-common/


### Building it yourself

### Protobuf definitions bundle

The required protobuf modules are referenced in `buf.yaml` and bundled in `generated-buf-build.binpb`

If you want to add more, you will need a (free) API token to access https://buf.build
The `make` command will automatically run the appropriate `buf build` command if needed.

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

```bash
substreams run -e mainnet.injective.streamingfast.io:443 substreams.yaml filtered_events -p filtered_events='type:injective.peggy.v1.EventDepositClaim' -s 9600 -t 9700
substreams run -e testnet.injective.streamingfast.io:443 substreams.yaml filtered_events -p filtered_events='type:injective.exchange.v1beta1.EventCancelSpotOrder' -s 27751658 -t +10
```

### Package it as .spkg

* optionally bump the version number in substreams.yaml

```bash
make package
```

## Modules

### all_events

The `all_events` module reads from the `sf.cosmos.type.v2.Block` source and outputs a list of all events in the block,
Some events are at the block level, others appear inside the transactions. The latter will include the transaction hash.

### filtered_events

The `filtered_events` module allows a reduction of the `all_events` output, only matching the events that match the 
requested type or include the attribute keys.
It pre-filters blocks using the `index_events` module for efficiency.

Use with parameters, ex:

```bash
substreams run [...] -p filtered_events="(type:message && attr:action) || (type:wasm && attr:_contract_address)"
```

### filtered_event_groups

The `filtered_event_groups` module reads from `all_events` and applies a filter on the event types and attribute keys, 
outputting *all the events* from transactions that have at least one event matching the filter.
It pre-filters blocks using the `index_events` module for efficiency.

Use with the same parameters as the `filtered_events` module.

### all_transactions

The `all_transactions` module extracts all the transactions from the Injective blockchain, providing useful information, such as _messages_, _signatures_ or _events_.

Some message types are parsed from their "Any" type into the the corresponding type of an enum. See ./proto/cosmos/v1/transactions.proto to see the ones that are supported.
The other types will still be shown as protobuf "Any" type.

### filtered_trx_by_events


The `filtered_trx_by_events` modules reads from `all_transactions` and applies a filter on the event types and attribute keys, 
outputting the complete transactions that have at least one event matching the filter.
It pre-filters blocks using the `index_events` module for efficiency.

Use with the same parameters as the `filtered_events` module.


### filtered_events_by_attribute_value

The `filtered_events_by_attribute_value` module reads from `all_events` and applies a filter on the event types, 
attribute keys and values, only outputting the events that match the filter. 

Use with parameters, ex:

```bash
substreams run [...] -p filtered_events_by_attribute_value="type:wasm && attr:_contract_address:inj1v77y5ttah96dc9qkcpc88ad7rce8n88e99t3m5"
```

For an optimized experience, we recommend importing this module in your own substreams.yaml, overloading 
the 'blockFilter' with a custom query to `index_events`, with the `use` parameter. See [an example](./derived-substreams-example.yaml)


### filtered_event_groups_by_attribute_value

The `filtered_events_groups_by_attribute_value` module reads from `all_events` and applies a filter on the event types,
attribute keys and values, outputting all the events from transactions that have at least one event matching the filter.

Use with the same parameters as the `filtered_events_by_attribute_value` module.

For an optimized experience, we recommend importing this module in your own substreams.yaml, overloading 
the 'blockFilter' with a custom query to `index_events`, with the `use` parameter. See [an example](./derived-substreams-example.yaml)


### filtered_trx_by_events_attribute_value

The `filtered_trx_by_events_attribute_value` module reads from `all_events` and applies a filter on the event types, 
attribute keys and values, outputting the complete transactions that have at least one event matching the filter.

Use with the same parameters as the `filtered_events_by_attribute_value` module.

For an optimized experience, we recommend importing this module in your own substreams.yaml, overloading 
the 'blockFilter' with a custom query to `index_events`, with the `use` parameter. See [an example](./derived-substreams-example.yaml)

