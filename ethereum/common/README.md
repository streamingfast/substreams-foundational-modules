## Substreams Ethereum Foundational Modules

Common Ethereum Substreams modules to filter transactions, with block indexes to skip
the blocks that cannot match.

### Modules

* `filtered_transactions` takes a query string as its parameter and returns the
  matching transactions, each with its receipt and calls. The same query drives the
  block filter, so blocks without a possible match are never read.

  Supported operators are logical or `||`, logical and `&&`, and parentheses `()`.
  Addresses and signatures are 0x-prefixed lowercase hexadecimal, prefixed by
  `evt_addr:`, `evt_sig:`, `call_to:`, `call_from:` or `call_method:`.

  ```
  ((evt_addr:0x1234… || evt_addr:0x5678…) && evt_sig:0xdeadbeef…) || call_to:0x0101…
  ```

* `index_events` is a block index over event signatures (`evt_sig:`) and event addresses
  (`evt_addr:`).

* `index_events_and_calls` emits the same event keys plus call keys: `call_to:`,
  `call_from:` and `call_method:`.

Use either index as the `blockFilter` of your own module to have the engine skip
blocks whose keys cannot satisfy your query.

A query cannot combine event keys with call keys using `&&`. The matcher is evaluated
against one log's keys, then one call's keys, so no single item carries both families and
`evt_sig:… && call_method:…` matches nothing. Use `||` to match either, and filter further
in your own module.

### Removed in v0.4.0

`all_events`, `all_calls`, `index_calls`, `filtered_events`, `filtered_calls` and
`filtered_events_and_calls` were removed. The two surviving indexes read the block
directly instead of going through a shared intermediate module, so the same keys are
produced without materialising every event and call of every block first.

If you consumed one of them:

| removed | use instead |
|---|---|
| `filtered_events`, `filtered_calls`, `filtered_events_and_calls` | `filtered_transactions`, whose query covers both event and call keys. Note it returns whole transaction traces, not a list of logs or calls, so a module that only read events now receives more data per match and should narrow it itself. |
| `index_calls` | `index_events_and_calls`, which emits the call keys unchanged |
| `all_events`, `all_calls` | read the block in your own module |

The surviving modules emit the same keys as before, but every module hash changed, so
cached state from v0.3.3 and earlier is not reused.
