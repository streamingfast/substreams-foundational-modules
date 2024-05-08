use crate::calls::*;
use crate::events::*;
use crate::pb::sf::substreams::ethereum::v1::{
    Call, Calls, Event, Events, EventsAndCalls, Transaction, Transactions,
};
use crate::pb::sf::substreams::index::v1::Keys;
use crate::pb::sf::substreams::v1::Clock;
use anyhow::Ok;
use std::collections::HashMap;
use substreams::errors::Error;
use substreams::Hex;
use substreams_ethereum::pb::eth::v2::{Block, Call as ethCall, Log};

#[substreams::handlers::map]
fn index_events_and_calls(events: Events, calls: Calls) -> Result<Keys, Error> {
    let mut keys = Keys::default();

    events.events.into_iter().for_each(|e| {
        if let Some(log) = e.log {
            evt_keys(&log).into_iter().for_each(|k| {
                keys.keys.push(k);
            });
        }
    });

    calls.calls.into_iter().for_each(|call| {
        if let Some(call) = &call.call {
            call_keys(call).into_iter().for_each(|k| {
                keys.keys.push(k);
            });
        }
    });

    Ok(keys)
}

#[substreams::handlers::map]
fn filtered_events_and_calls(
    query: String,
    events: Events,
    calls: Calls,
) -> Result<EventsAndCalls, Error> {
    let filtered_calls: Vec<Call> = calls
        .calls
        .into_iter()
        .filter(|e| {
            if let Some(call) = &e.call {
                call_matches(call, &query)
            } else {
                false
            }
        })
        .collect();

    let filtered_events: Vec<Event> = events
        .events
        .into_iter()
        .filter(|e| {
            if let Some(log) = &e.log {
                evt_matches(log, &query)
            } else {
                false
            }
        })
        .collect();

    Ok(EventsAndCalls {
        events: filtered_events,
        calls: filtered_calls,
        clock: calls.clock,
    })
}

#[substreams::handlers::map]
fn filtered_transactions(query: String, block: Block) -> Result<Transactions, Error> {
    let mut events: HashMap<String, Vec<&Log>> = HashMap::new();
    block.logs().for_each(|log| {
        let k = Hex::encode(&log.receipt.transaction.hash);
        events.entry(k).or_default().push(log.log);
    });

    let mut calls: HashMap<String, Vec<&ethCall>> = HashMap::new();
    block.calls().for_each(|call| {
        let k = Hex::encode(&call.transaction.hash);
        calls.entry(k).or_default().push(call.call);
    });

    let filtered: Vec<Transaction> = block
        .transaction_traces
        .iter()
        .filter(|tx| tx.status == 1)
        .filter(|tt| {
            let mut matched = false;
            let hash = Hex::encode(&tt.hash);
            if let Some(ev) = events.get(&hash) {
                ev.iter().for_each(|log| {
                    if evt_matches(&log, &query) {
                        matched = true;
                        return;
                    }
                });
            };
            if let Some(ca) = calls.get(&hash) {
                ca.iter().for_each(|call| {
                    if call_matches(&call, &query) {
                        matched = true;
                        return;
                    };
                });
            };

            matched
        })
        .map(|tt| {
            let hash = Hex::encode(&tt.hash);
            Transaction {
                trace: Some(tt.to_owned()),
                tx_hash: hash,
            }
        })
        .collect();

    let clock = Some(Clock {
        timestamp: Some(block.header.unwrap().timestamp.unwrap()),
        id: Hex::encode(&block.hash),
        number: block.number,
    });

    Ok(Transactions {
        transactions: filtered,
        clock: clock,
        detail_level: block.detail_level,
    })
}
