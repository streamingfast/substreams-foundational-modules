# USDT Exchange Volume

This is a simple Substreams that tracks the amount of USDT exchanged in the `INJ-USDT` pair in the Dojo DEX (https://dojo.trading).

The Substreams tracks all the transactions where USDT is exchanged and emits a `USDTExchange` with the amount of USDT that was traded in the transaction.

Specifically, the Substreams loops of all the transactions looking for messages of type `cosmwasm.wasm.v1.MsgExecuteContract` and `injective.wasmx.v1.MsgExecuteContractCompat`. Then, it extracts the necessary information from the events (logs) of the transactions.

There are two relevant event types:
- `execute`: this event contains the smart contract address where the transactions was executed. The Substreams only tracks swaps on the `INJ-USDT` pair smart contract.
- `wasm`: this event contains the swap information.

You can find the transactions for the `INJ-USDT` pair in the [official Dojo website](https://dojo.trading/pairs/inj1h0mpv48ctcsmydymh2hnkal7hla5gl4gftemqv).

## Getting Started

### Generate Protos
```bash
make protogen
```

### Build Substreams
```bash
make build
```

### Run Substreams
```bash
substreams run substreams.yaml map_usdt_exchanges -e mainnet.injective.streamingfast.io:443 --start-block=55507159--stop-block=+200
```