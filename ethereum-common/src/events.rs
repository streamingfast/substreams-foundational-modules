use std::collections::HashSet;

use crate::pb::sf::substreams::ethereum::v1::{Event, Events};
use crate::pb::sf::substreams::v1::Clock;
use anyhow::Ok;
use substreams::errors::Error;
use substreams::matches_keys_in_parsed_expr;
use substreams::pb::sf::substreams::index::v1::Keys;
use substreams::Hex;
use substreams_ethereum::pb::eth::v2::Block;

#[substreams::handlers::map]
fn all_events(blk: Block) -> Result<Events, Error> {
    let clock = Clock {
        timestamp: Some(blk.header.unwrap().timestamp.unwrap()),
        id: Hex::encode(&blk.hash),
        number: blk.number,
    };

    let events: Vec<Event> = blk
        .transaction_traces
        .into_iter()
        .filter(|tx| tx.status == 1)
        .map(|tx| (tx.receipt.unwrap_or_default(), tx.hash))
        .map(|(receipt, hash)| (receipt.logs, hash))
        .flat_map(|(log, hash)| {
            log.into_iter().map(move |l| Event {
                tx_hash: Hex::encode(&hash),
                log: Some(l),
            })
        })
        .collect();

    Ok(Events {
        events: events,
        clock: Some(clock),
    })
}

#[substreams::handlers::map]
fn index_events(events: Events) -> Result<Keys, Error> {
    let keys: Vec<_> = events
        .events
        .iter()
        .filter_map(|event| event.log.as_ref())
        .flat_map(evt_keys)
        .collect::<HashSet<_>>()
        .into_iter()
        .collect();

    Ok(Keys { keys })
}

#[substreams::handlers::map]
fn filtered_events(query: String, events: Events) -> Result<Events, Error> {
    let filtered: Vec<Event> = events
        .events
        .into_iter()
        .filter(|e| {
            if let Some(log) = &e.log {
                evt_matches(log, &query).expect("matching calls from query")
            } else {
                false
            }
        })
        .collect();

    Ok(Events {
        events: filtered,
        clock: events.clock,
    })
}

pub fn evt_keys(log: &substreams_ethereum::pb::eth::v2::Log) -> Vec<String> {
    let mut keys = Vec::new();

    if log.topics.len() > 0 {
        let k_log_sign = format!("evt_sig:0x{}", Hex::encode(log.topics.get(0).unwrap()));
        keys.push(k_log_sign);
    }

    let k_log_address = format!("evt_addr:0x{}", Hex::encode(&log.address));
    keys.push(k_log_address);

    keys
}

pub fn evt_matches(
    log: &substreams_ethereum::pb::eth::v2::Log,
    query: &str,
) -> Result<bool, Error> {
    matches_keys_in_parsed_expr(&evt_keys(log), query)
}
