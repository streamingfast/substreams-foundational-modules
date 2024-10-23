# Mantra Foundational Modules

Common Mantra Substreams modules to extract events and transactions with indexing
This package inherits from the Injective Foundational Modules, which share the Cosmos Block model.

Use one of those optimized modules with a query string as a parameter:
* filtered_events
* filtered_event_groups

The query string will be used for the blockfilter as well as the actual filtering of the events/transactions

The following modules are also available for more specific filtering, but they do not use an index by default:
* filtered_events_by_attribute_value
* filtered_event_groups_by_attribute_value