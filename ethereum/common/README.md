## Substreams Ethereum Foundational Modules

Common Ethereum Substreams modules to extract events, calls and transactions, with block
indexes to skip the blocks that cannot match.

### Filtered modules

Each takes a query string as its parameter and returns only the matching items. The same
query drives the block filter, so blocks without a possible match are never read.

* `filtered_events` matches on event address (`evt_addr:`) and signature (`evt_sig:`)
* `filtered_calls` matches on called contract (`call_to:`), caller (`call_from:`) and
  method signature (`call_method:`)
* `filtered_events_and_calls` matches on either family and returns both
* `filtered_transactions` matches on either family and returns the whole transaction
  trace of each match, receipt and calls included

Supported operators are logical or `||`, logical and `&&`, and parentheses `()`.
Addresses and signatures are 0x-prefixed lowercase hexadecimal.

```
((evt_addr:0x1234… || evt_addr:0x5678…) && evt_sig:0xdeadbeef…) || call_to:0x0101…
```

A query cannot combine event keys with call keys using `&&`. The matcher is evaluated
against one log's keys, then one call's keys, so no single item carries both families and
`evt_sig:… && call_method:…` matches nothing. Use `||` to match either.

### Block indexes

Use one as the `blockFilter` of your own module to have the engine skip blocks whose keys
cannot satisfy your query.

* `index_events` emits `evt_sig:` and `evt_addr:`
* `index_calls` emits `call_to:`, `call_from:` and `call_method:`
* `index_events_and_calls` emits all five

### Removed in v0.4.0

`all_events` and `all_calls` were removed. They materialised every event and every call of
every block so that the modules above could read them, which held the same data twice and
wrote it to the object store on the way through. Those modules now read the block directly
and lazily, decoding only what a query matches, and emit exactly what they emitted before.

If you consumed `all_events` or `all_calls` directly, read the block in your own module, or
use `filtered_events` / `filtered_calls` with a query.

Every module hash changed, so cached state from v0.3.3 and earlier is not reused.
