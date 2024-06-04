# Description

Foundational Substreams modules for various blockchains. Use them directly to quickly get the data that you want or build your own substreams that depend on them! 

# Context

Substreams modules read its data from "flat files", either "block files" (saved in bundles of 100) or from the cached outputs of the previous modules (usually saved in bundles of 1000, but that is configurable server-side).
Reducing the amount of data that is read and processed in the WASM module has the most significant impact on performance and costs.
Therefore, creating and pre-caching "well-known" modules with commonly used sets of data can benefit a lot of substreams.
Additionally, creating and pre-caching "index modules" can be used to further reduce the number of blocks that will be actually processed.

Foundational modules can contain:

- mappers that extract the essential data from blocks with minimal transformation, ex:
  - "log/events" (with added trx hash) in Ethereum 
  - "instructions" (with flattened addresses) in Solana
- mappers that filter out unwanted data from blocks ("empty" or "spammy" transactions), ex:
  - removing the solana instructions that are only "votes"
- block indexes that adds "keys" to blocks, ex: one label for each "account/actor" that generated an event (ex: evt_from:0x01234, call_addr:0x9876, etc.)
- mappers that help with the use of the block indexes by accepting a `param` and also using it as a `blockFilter.queryString`, like this:
  1. read the output from the "ethereum events" mapper
  2. use the blockFilter attribute to only process the blocks that match a few keys (ex: `evt_from:0xb0b || evt_from:0xa11ce`) 
  3. from all the events that come from those blocks, apply the same filtering logic (by extracting the same keys and applying the query from the params), only outputting the events that match the `param`.

# Current implementations

* ethereum-common
* antelope-common
* injective-common
