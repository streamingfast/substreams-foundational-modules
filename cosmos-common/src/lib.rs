#[allow(dead_code, unused_imports)]
mod pb;

use std::collections::HashMap;

use crate::pb::sf::cosmos::r#type::v2::BlockLazyView;
use anyhow::anyhow;
use buffa::view::{LazyMessageView, MessageView};
use pb::sf::substreams::cosmos::v1::*;
use pb::sf::substreams::v1::Clock;
use sha2::{Digest, Sha256};
use substreams::errors::Error;
use substreams::pb::sf::substreams::index::v1::Keys;

#[substreams::handlers::map]
pub fn all_events(block: &BlockLazyView<'_>) -> Result<EventList, Error> {
    // Mutable list to add the output of the Substreams
    let mut events: Vec<Event> = Vec::new();

    if block.txs.len() != block.tx_results.len() {
        return Err(anyhow!("Transaction list and result list do not match"));
    }

    // block events are the combination of BeginBlockEvents and EndBlockEvents
    for event in block.events.iter() {
        events.push(Event {
            event: event?.to_owned_message()?.into(),
            transaction_hash: "".to_string(),
        });
    }

    for (i, tx_result) in block.tx_results.iter().enumerate() {
        let tx_result = tx_result?;
        let tx_hash = compute_tx_hash(block.txs.get(i).unwrap());

        for event in tx_result.events.iter() {
            events.push(Event {
                event: event?.to_owned_message()?.into(),
                transaction_hash: tx_hash.clone(),
            });
        }
    }

    Ok(EventList {
        events: events,
        clock: Clock {
            id: hex::encode(block.hash),
            number: block.height as u64,
            timestamp: block
                .time
                .as_option()
                .map(|t| t.to_owned_message())
                .transpose()?
                .into(),
        }
        .into(),
    })
}

#[substreams::handlers::map]
fn index_events(events: EventList) -> Result<Keys, Error> {
    let mut keys = Keys::default();

    events.events.into_iter().for_each(|e| {
        if let Some(ev) = e.event.into_option() {
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
    let matcher = substreams::sqe::expr_matcher(&query);

    let filtered: Vec<Event> = events
        .events
        .into_iter()
        .filter(|e| {
            if let Some(ev) = e.event.as_option() {
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
    let matcher = substreams::sqe::expr_matcher(&query);

    let matching_trx_hashes = events
        .events
        .iter()
        .filter(|e| {
            if let Some(ev) = e.event.as_option() {
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
    let matcher = substreams::sqe::expr_matcher(&query);

    let filtered: Vec<Event> = events
        .events
        .into_iter()
        .filter(|e| {
            if let Some(ev) = e.event.as_option() {
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
    let matcher = substreams::sqe::expr_matcher(&query);

    let matching_trx_hashes = events
        .events
        .iter()
        .filter(|e| {
            if let Some(ev) = e.event.as_option() {
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
        let bytes =
            testing::read_block_bytes("./src/testdata/injective_mainnet_103863031.binpb.base64");
        let block = BlockLazyView::decode_lazy(&bytes).expect("valid block");

        // When
        let all_events = substreams::testing::map!(all_events(&block)).unwrap();
        let result =
            substreams::testing::map!(filtered_events("type:transfer".to_owned(), all_events));

        // Expect
        let result_events = result.unwrap().events;

        assert!(result_events.len() > 0);
        result_events
            .iter()
            .for_each(|event| assert_eq!(event.event.r#type, "transfer"));
    }

    #[test]
    fn test_filtered_event_groups() {
        // Given
        let bytes =
            testing::read_block_bytes("./src/testdata/injective_mainnet_103863031.binpb.base64");
        let block = BlockLazyView::decode_lazy(&bytes).expect("valid block");

        // When
        let all_events = substreams::testing::map!(all_events(&block)).unwrap();
        let result = substreams::testing::map!(filtered_event_groups(
            "type:transfer".to_owned(),
            all_events
        ));

        // Expect
        let result_events = result.unwrap().events;

        assert!(result_events.len() > 0);
        result_events.iter().for_each(|event| {
            let inner_event = &event.event;

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
        let bytes =
            testing::read_block_bytes("./src/testdata/injective_mainnet_103863031.binpb.base64");
        let block = BlockLazyView::decode_lazy(&bytes).expect("valid block");

        // When
        let all_events = substreams::testing::map!(all_events(&block)).unwrap();
        let result = substreams::testing::map!(filtered_events_by_attribute_value(
            "type:transfer && attr:sender:inj14vnmw2wee3xtrsqfvpcqg35jg9v7j2vdpzx0kk".to_owned(),
            all_events,
        ));

        // Expect
        let result_events = result.unwrap().events;

        assert!(result_events.len() > 0);
        result_events.iter().for_each(|event| {
            let inner_event = &event.event;

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
        let bytes =
            testing::read_block_bytes("./src/testdata/injective_mainnet_103863031.binpb.base64");
        let block = BlockLazyView::decode_lazy(&bytes).expect("valid block");

        // When
        let all_events = substreams::testing::map!(all_events(&block)).unwrap();
        let result = substreams::testing::map!(filtered_event_groups_by_attribute_value(
            "type:transfer && attr:sender".to_owned(),
            all_events,
        ));

        // Expect
        let result_events = result.unwrap().events;

        assert!(result_events.len() > 0);
        result_events.iter().for_each(|event| {
            let inner_event = &event.event;

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
