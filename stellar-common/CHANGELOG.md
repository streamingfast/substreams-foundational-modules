# Change log

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/), and this
project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html). See [MAINTAINERS.md](./MAINTAINERS.md)
for instructions to keep up to date.

## Unreleased

### Fixed
- Fix `map_events` WASM entrypoint crash ("too many arguments provided") by correcting its manifest inputs from `params: string` + `source: sf.stellar.type.v1.Block` to `map: map_transactions`, matching the actual Rust function signature.
- Fix `map_events` transaction decode errors on mainnet: the firehose-stellar node no longer populates `result_meta_xdr`; events are now decoded from the pre-extracted `transaction.events.contract_events_xdr` XDR bytes instead.
- Upgrade `stellar-xdr` from `25.0` to `27.0` to match the current Stellar protocol XDR definitions.
- Remove `result_meta_xdr` from the `Transaction` proto type and all generated code to match the upstream firehose-stellar v1.0.6 proto.
- Switch Firehose Stellar block descriptor from a local `.binpb` file to `buf.build/streamingfast/firehose-stellar:v1.0.6`.

## v0.5.0

* Release v0.5.0

## v0.4.0

* Added indexing of Stellar event's topics.
