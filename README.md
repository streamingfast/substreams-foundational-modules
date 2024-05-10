# Description

Foundational Substreams modules for various blockchains. Use them directly to quickly get the data that you want or build your own substreams that depend on them! 

# Context

Substreams modules read its data from "flat files", either "block files" (saved in bundles of 100) or from the cached outputs of the previous modules (usually saved in bundles of 1000, but that is configurable server-side).
Reducing the amount of data that is read and processed in the WASM module has the most significant impact on performance and costs.
Therefore, creating and pre-caching "well-known" modules with commonly used sets of data can benefit a lot of substreams.
Additionally, creating and pre-caching "index modules" can be used to further reduce the number of blocks that will be actually processed.

Foundational modules can contain:
1) mappers that extract the essential data from blocks ("log/events" in Ethereum, instructions in Solana )
2) mappers that filter out unwanted data from blocks ("empty" or "spammy" transactions)
3) block indexes that adds "labels" to blocks, ex: one label for each "account/actor" that generated an event, one label for each contract that executed code, etc.

# Current implementations

* ethereum-common
