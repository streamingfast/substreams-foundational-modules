# mantra-common

Common Mantra Substreams modules to extract events and transactions with indexing
This package inherits from the generic Cosmos Foundational Modules.

## Usage

Usually, foundational modules are directly imported and used in other Substreams. All the official foundational modules are stored in [substreams.dev](`https://substreams.dev`).

```yaml
specVersion: v0.1.0
package:
  name: my_project
  version: v0.1.0

imports:
  mantra: https://spkg.io/streamingfast/mantra-common-v0.2.0.spkg # Import the package from substreams.dev

modules:
  - name: my_events # Define your Substreams module
    use: mantra:filtered_events # Use the imported package
    initialBlock: 70000000

params:
  my_events: "(type:message && attr:action) || (type:wasm && attr:_contract_address)" # Pass the filter as parameter to the module
```

To run the Substreams:

```bash
substreams build
substreams auth
substreams gui
```

## Modules

### all_events (map)

Retrieves all the events in the Mantra blockchain without any filtering.

### index_events (index)

The module create an index (a cache) to efficiently retrieve events by their type and/or attribute keys.

The module sets the keys corresponding to all event 'types' and 'attribute keys' in the block For example: `type:coin_received`, `attr:action`, `attr:sender` ...

The attribute values are never indexed because they have a high cardinality and would be too expensive to index.

### filtered_events (map)

The module reads from `all_events` and applies a filter on the event types and attribute keys, only outputing the events that match the filter.

The filter is specificed in the parameters of the module.

```yaml
...

params:
  filtered_events: "(type:rewards && attr:validator)"
```

### filtered_event_groups (map)

The module reads from `all_events` and applies a filter on the event types and attribute keys, outputing all the events from transactions that have at least one event matching the filter.

```yaml
params:
    filtered_event_groups: "type:rewards && attr:validator"
```

### filtered_events_by_attribute_value (map)

The module reads from `all_events` and applies a filter on the event types, attribute keys and values, only outputing the events that match the filter.

**NOTE:** This module does not use the index created by `index_events`.

```yaml
params:
    filtered_events_by_attribute_value: "type:rewards && attr:validator:mantravaloper18se5kq0z86pqfym8uuuqp77kyd788npj3wx7fc"
```

### filtered_event_groups_by_attribute_value (map)

The module reads from `all_events` and applies a filter on the event types, attribute keys and values, outputing all the events from transactions that have at least one event matching the filter.

**NOTE:** This module does not use the index created by `index_events`.

```yaml
params:
    filtered_event_groups_by_attribute_value: "type:rewards && attr:validator:mantravaloper18se5kq0z86pqfym8uuuqp77kyd788npj3wx7fc"
```