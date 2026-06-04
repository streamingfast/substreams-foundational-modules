# Change log

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/), and this
project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html). See [MAINTAINERS.md](./MAINTAINERS.md)
for instructions to keep up to date.

## Unreleased

### Fixed
- Fix `map_events` WASM entrypoint crash ("too many arguments provided") by correcting its manifest inputs from `params: string` + `source: sf.stellar.type.v1.Block` to `map: map_transactions`, matching the actual Rust function signature.

## v0.5.0

* Release v0.5.0

## v0.4.0

* Added indexing of Stellar event's topics.
