#[allow(dead_code)]
mod pb;

use std::collections::HashMap;

use crate::pb::sf::cosmos::r#type::v2::Block;
use anyhow::anyhow;
use pb::sf::substreams::cosmos::v1::*;
use pb::sf::substreams::v1::Clock;
use sha2::{Digest, Sha256};
use substreams::errors::Error;
use substreams::pb::sf::substreams::index::v1::Keys;

#[substreams::handlers::map]
pub fn all_events(block: Block) -> Result<EventList, Error> {
    _all_events(block)
}

/// _all_events is equal to [all_events] but exists only for unit testing purposes.
pub fn _all_events(block: Block) -> Result<EventList, Error> {
    // Mutable list to add the output of the Substreams
    let mut events: Vec<Event> = Vec::new();

    if block.txs.len() != block.tx_results.len() {
        return Err(anyhow!("Transaction list and result list do not match"));
    }

    // block events are the combination of BeginBlockEvents and EndBlockEvents
    events.extend(block.events.into_iter().map(|event| {
        return Event {
            event: Some(event),
            transaction_hash: "".to_string(),
        };
    }));

    for (i, tx_result) in block.tx_results.into_iter().enumerate() {
        let tx_hash = compute_tx_hash(block.txs.get(i).unwrap());

        let block_events: Vec<Event> = tx_result
            .events
            .into_iter()
            .map(|event| {
                return Event {
                    event: Some(event),
                    transaction_hash: tx_hash.clone(),
                };
            })
            .collect();

        events.extend(block_events);
    }

    Ok(EventList {
        events: events,
        clock: Some(Clock {
            id: hex::encode(block.hash),
            number: block.height as u64,
            timestamp: block.time,
        }),
    })
}

#[substreams::handlers::map]
fn index_events(events: EventList) -> Result<Keys, Error> {
    let mut keys = Keys::default();

    events.events.into_iter().for_each(|e| {
        if let Some(ev) = e.event {
            keys.keys.push(format!("type:{}", ev.r#type));
            ev.attributes.into_iter().for_each(|attr| {
                keys.keys.push(format!("attr:{}", attr.key));
            });
        }
    });

    Ok(keys)
}

#[substreams::handlers::map]
fn filtered_events(query: String, events: EventList) -> Result<EventList, Error> {
    _filtered_events(query, events)
}

/// _filtered_events is equal to [filtered_events] but exists only for unit testing purposes.
fn _filtered_events(query: String, events: EventList) -> Result<EventList, Error> {
    let matcher: substreams::ExprMatcher<'_> = substreams::expr_matcher(&query);

    let filtered: Vec<Event> = events
        .events
        .into_iter()
        .filter(|e| {
            if let Some(ev) = &e.event {
                let mut keys = Vec::new();
                keys.push(format!("type:{}", ev.r#type.clone()));
                ev.attributes.iter().for_each(|attr| {
                    keys.push(format!("attr:{}", attr.key));
                });

                matcher.matches_keys(&keys)
            } else {
                false
            }
        })
        .collect();

    if filtered.len() == 0 {
        return Ok(EventList::default());
    }
    Ok(EventList {
        events: filtered,
        clock: events.clock,
    })
}

#[substreams::handlers::map]
fn filtered_event_groups(query: String, events: EventList) -> Result<EventList, Error> {
    _filtered_event_groups(query, events)
}

/// _filtered_event_groups is equal to [filtered_event_groups] but exists only for unit testing purposes.
fn _filtered_event_groups(query: String, events: EventList) -> Result<EventList, Error> {
    let matcher: substreams::ExprMatcher<'_> = substreams::expr_matcher(&query);

    let matching_trx_hashes = events
        .events
        .iter()
        .filter(|e| {
            if let Some(ev) = &e.event {
                let mut keys = Vec::new();
                keys.push(format!("type:{}", ev.r#type.clone()));
                ev.attributes.iter().for_each(|attr| {
                    keys.push(format!("attr:{}", attr.key));
                });

                matcher.matches_keys(&keys)
            } else {
                false
            }
        })
        .map(|e| (e.transaction_hash.to_string(), true))
        .collect::<HashMap<String, bool>>();

    let filtered: Vec<Event> = events
        .events
        .into_iter()
        .filter(|e| matching_trx_hashes.contains_key(e.transaction_hash.as_str()))
        .collect();

    if filtered.len() == 0 {
        return Ok(EventList::default());
    }
    Ok(EventList {
        events: filtered,
        clock: events.clock,
    })
}

#[substreams::handlers::map]
fn filtered_events_by_attribute_value(
    query: String,
    events: EventList,
) -> Result<EventList, Error> {
    _filtered_events_by_attribute_value(query, events)
}

/// _filtered_events_by_attribute_value is equal to [filtered_events_by_attribute_value] but exists only for unit testing purposes.
fn _filtered_events_by_attribute_value(
    query: String,
    events: EventList,
) -> Result<EventList, Error> {
    let matcher: substreams::ExprMatcher<'_> = substreams::expr_matcher(&query);

    let filtered: Vec<Event> = events
        .events
        .into_iter()
        .filter(|e| {
            if let Some(ev) = &e.event {
                let mut keys = Vec::new();
                keys.push(format!("type:{}", ev.r#type.clone()));
                ev.attributes.iter().for_each(|attr| {
                    keys.push(format!("attr:{}", attr.key));
                    keys.push(format!("attr:{}:{}", attr.key, attr.value));
                });

                matcher.matches_keys(&keys)
            } else {
                false
            }
        })
        .collect();

    if filtered.len() == 0 {
        return Ok(EventList::default());
    }
    Ok(EventList {
        events: filtered,
        clock: events.clock,
    })
}

#[substreams::handlers::map]
fn filtered_event_groups_by_attribute_value(
    query: String,
    events: EventList,
) -> Result<EventList, Error> {
    _filtered_event_groups_by_attribute_value(query, events)
}

/// _filtered_event_groups_by_attribute_value is equal to [filtered_event_groups_by_attribute_value] but exists only for unit testing purposes.
fn _filtered_event_groups_by_attribute_value(
    query: String,
    events: EventList,
) -> Result<EventList, Error> {
    let matcher: substreams::ExprMatcher<'_> = substreams::expr_matcher(&query);

    let matching_trx_hashes = events
        .events
        .iter()
        .filter(|e| {
            if let Some(ev) = &e.event {
                let mut keys = Vec::new();
                keys.push(format!("type:{}", ev.r#type.clone()));
                ev.attributes.iter().for_each(|attr| {
                    keys.push(format!("attr:{}", attr.key));
                    keys.push(format!("attr:{}:{}", attr.key, attr.value));
                });

                matcher.matches_keys(&keys)
            } else {
                false
            }
        })
        .map(|e| (e.transaction_hash.to_string(), true))
        .collect::<HashMap<String, bool>>();

    let filtered: Vec<Event> = events
        .events
        .into_iter()
        .filter(|e| matching_trx_hashes.contains_key(e.transaction_hash.as_str()))
        .collect();

    if filtered.len() == 0 {
        return Ok(EventList::default());
    }
    Ok(EventList {
        events: filtered,
        clock: events.clock,
    })
}

fn compute_tx_hash(tx_as_bytes: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(tx_as_bytes);
    let tx_hash = hasher.finalize();
    return hex::encode(tx_hash);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_filtered_events() {
        // Given
        let block = testing::read_block("./src/testdata/injective_mainnet_103863031.binpb.base64");

        // When
        let result = _filtered_events("type:transfer".to_owned(), _all_events(block).unwrap());

        // Expect
        let result_events = result.unwrap().events;

        assert!(result_events.len() > 0);
        result_events
            .iter()
            .for_each(|event| assert_eq!(event.event.as_ref().unwrap().r#type, "transfer"));
    }

    #[test]
    fn test_filtered_event_groups() {
        // Given
        let block = testing::read_block("./src/testdata/injective_mainnet_103863031.binpb.base64");

        // When
        let result =
            _filtered_event_groups("type:transfer".to_owned(), _all_events(block).unwrap());

        // Expect
        let result_events = result.unwrap().events;

        assert!(result_events.len() > 0);
        result_events.iter().for_each(|event| {
            let inner_event = event.event.as_ref().unwrap();

            if inner_event.r#type == "transfer" {
                assert_eq!(
                    inner_event
                        .attributes
                        .iter()
                        .any(|attr| attr.key == "sender"),
                    true
                )
            }
        });
    }

    #[test]
    fn test_filtered_event_by_attribute_value() {
        // Given
        let block = testing::read_block("./src/testdata/injective_mainnet_103863031.binpb.base64");

        // When
        let result = _filtered_events_by_attribute_value(
            "type:transfer && attr:sender:inj14vnmw2wee3xtrsqfvpcqg35jg9v7j2vdpzx0kk".to_owned(),
            _all_events(block).unwrap(),
        );

        // Expect
        let result_events = result.unwrap().events;

        assert!(result_events.len() > 0);
        result_events.iter().for_each(|event| {
            let inner_event = event.event.as_ref().unwrap();

            assert_eq!(inner_event.r#type, "transfer");
            assert_eq!(
                inner_event
                    .attributes
                    .iter()
                    .any(|attr| attr.key == "sender"
                        && attr.value == "inj14vnmw2wee3xtrsqfvpcqg35jg9v7j2vdpzx0kk"),
                true
            )
        });
    }

    #[test]
    fn test_filtered_event_groups_by_attribute_value() {
        // Given
        let block = testing::read_block("./src/testdata/injective_mainnet_103863031.binpb.base64");

        // When
        let result = _filtered_event_groups_by_attribute_value(
            "type:transfer && attr:sender".to_owned(),
            _all_events(block).unwrap(),
        );

        // Expect
        let result_events = result.unwrap().events;

        assert!(result_events.len() > 0);
        result_events.iter().for_each(|event| {
            let inner_event = event.event.as_ref().unwrap();

            if inner_event.r#type == "transfer" {
                assert_eq!(
                    inner_event
                        .attributes
                        .iter()
                        .any(|attr| attr.key == "sender"),
                    true
                )
            }
        });
    }
}
