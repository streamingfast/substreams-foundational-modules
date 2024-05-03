use crate::pb::sf::substreams::ethereum::v1::{
    Call, Calls, Event, Events, EventsAndCalls, Transaction, Transactions,
};
use crate::pb::sf::substreams::index::v1::Keys;
use crate::pb::sf::substreams::v1::Clock;
use anyhow::Ok;
use regex::Regex;
use std::collections::HashMap;
use substreams::errors::Error;
use substreams::Hex;
use substreams_ethereum::pb::eth::v2::{Block, Call as ethCall, Log};

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
fn all_calls(blk: Block) -> Result<Calls, Error> {
    let clock = Clock {
        timestamp: Some(blk.header.unwrap().timestamp.unwrap()),
        id: Hex::encode(&blk.hash),
        number: blk.number,
    };

    let calls: Vec<Call> = blk
        .transaction_traces
        .into_iter()
        .filter(|tx| tx.status == 1)
        .map(|tx| (tx.calls, tx.hash))
        .flat_map(|(call, hash)| {
            call.into_iter().map(move |c| Call {
                tx_hash: Hex::encode(&hash),
                call: Some(c),
            })
        })
        .collect();


    Ok(Calls {
        calls: calls,
        clock: Some(clock),
    })
}

fn evt_keys(log: &substreams_ethereum::pb::eth::v2::Log) -> Vec<String> {
    let mut keys = Vec::new();

    if log.topics.len() > 0 {
        let k_log_sign = format!("evt_sig:0x{}", Hex::encode(log.topics.get(0).unwrap()));
        keys.push(k_log_sign);
    }

    let k_log_address = format!("evt_addr:0x{}", Hex::encode(&log.address));
    keys.push(k_log_address);

    keys
}

fn call_keys(call: &substreams_ethereum::pb::eth::v2::Call) -> Vec<String> {
    let mut keys = Vec::new();

    let from_bytes = &call.caller;
    let k_call_from = format!("call_from:0x{}", Hex::encode(from_bytes));
    keys.push(k_call_from);

    let to_bytes = &call.address;
    let k_call_to = format!("call_to:0x{}", Hex::encode(to_bytes));
    keys.push(k_call_to);

    let input_bytes = &call.input;

    if input_bytes.len() >= 4 {
        let k_call_method = format!("call_method:0x{}", Hex::encode(&input_bytes[..4]));
        keys.push(k_call_method);
    }

    keys
}

#[substreams::handlers::map]
fn index_call_and_events(blk: Block) -> Result<Keys, Error> {
    let mut keys = Keys::default();

    blk.logs().into_iter().for_each(|log| {
        evt_keys(&log.log).into_iter().for_each(|k| {
            keys.keys.push(k);
        });
    });

    blk.calls().into_iter().for_each(|call| {
        call_keys(&call.call).into_iter().for_each(|k| {
            keys.keys.push(k);
        });
    });

    Ok(keys)
}

#[substreams::handlers::map]
fn filtered_events(query: String, events: Events) -> Result<Events, Error> {
    // TODO: implement full query parsing
    // this currently does a simple match on any term, regardless of the parenthesis or && operators
    // "(evt_addr:0xb0b || evt_addr:0xa11ce) && evt_sig:0xbeefbeef
    // will be interpreted as "evt_addr:0xb0b || evt_addr:0xa11ce || evt_sig:0xbeefbeef"

    let re = Regex::new(r"\bevt[\w]+:[\w]+\b").unwrap();

    let tokens: HashMap<&str, bool> = re
        .captures_iter(query.as_str())
        .map(|c| (c.get(0).unwrap().as_str(), true))
        .collect();

    let filtered: Vec<Event> = events
        .events
        .into_iter()
        .filter(|e| {
            if let Some(log) = &e.log {
                evt_matches(log, &tokens)
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

fn evt_matches(log: &substreams_ethereum::pb::eth::v2::Log, query: &HashMap<&str, bool>) -> bool {
    let mut matched = false;
    evt_keys(log).into_iter().for_each(|k| {
        if let Some(_) = query.get(k.as_str()) {
            matched = true;
            return;
        }
    });
    matched
}

fn call_matches(
    call: &substreams_ethereum::pb::eth::v2::Call,
    query: &HashMap<&str, bool>,
) -> bool {
    let mut matched = false;
    call_keys(call).into_iter().for_each(|k| {
        if let Some(_) = query.get(k.as_str()) {
            matched = true;
            return;
        }
    });
    matched
}

#[substreams::handlers::map]
fn filtered_calls(query: String, calls: Calls) -> Result<Calls, Error> {
    // TODO: implement full query parsing
    // this currently does a simple match on any term, regardless of the parenthesis or && operators
    // "(call_from:0xb0b || call_from:0xa11ce) && call_method:0xbeefbeef
    // will be interpreted as "call_from:0xb0b || call_from:0xa11ce || call_method:0xbeefbeef

    let re = Regex::new(r"\bcall[\w]+:[\w]+\b").unwrap();

    let tokens: HashMap<&str, bool> = re
        .captures_iter(query.as_str())
        .map(|c| (c.get(0).unwrap().as_str(), true))
        .collect();

    let filtered: Vec<Call> = calls
        .calls
        .into_iter()
        .filter(|e| {
            if let Some(call) = &e.call {
                call_matches(call, &tokens)
            } else {
                false
            }
        })
        .collect();

    Ok(Calls {
        calls: filtered,
        clock: calls.clock,
    })
}

#[substreams::handlers::map]
fn filtered_events_and_calls(query: String, events: Events, calls: Calls) -> Result<EventsAndCalls, Error> {
    // TODO: implement full query parsing
    // this currently does a simple match on any term, regardless of the parenthesis or && operators
    // "(call_from:0xb0b || call_from:0xa11ce) && call_method:0xbeefbeef
    // will be interpreted as "call_from:0xb0b || call_from:0xa11ce || call_method:0xbeefbeef

    let evt_re = Regex::new(r"\bevt[\w]+:[\w]+\b").unwrap();
    let call_re = Regex::new(r"\bcall[\w]+:[\w]+\b").unwrap();

    let evt_tokens: HashMap<&str, bool> = evt_re
        .captures_iter(query.as_str())
        .map(|c| (c.get(0).unwrap().as_str(), true))
        .collect();

    let call_tokens: HashMap<&str, bool> = call_re
        .captures_iter(query.as_str())
        .map(|c| (c.get(0).unwrap().as_str(), true))
        .collect();

    let filtered_calls: Vec<Call> = calls
        .calls
        .into_iter()
        .filter(|e| {
            if let Some(call) = &e.call {
                call_matches(call, &call_tokens)
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
                evt_matches(log, &evt_tokens)
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
    // TODO: implement full query parsing
    // this currently does a simple match on any term, regardless of the parenthesis or && operators

    let evt_re = Regex::new(r"\bevt[\w]+:[\w]+\b").unwrap();
    let call_re = Regex::new(r"\bcall[\w]+:[\w]+\b").unwrap();

    let evt_tokens: HashMap<&str, bool> = evt_re
        .captures_iter(query.as_str())
        .map(|c| (c.get(0).unwrap().as_str(), true))
        .collect();

    let call_tokens: HashMap<&str, bool> = call_re
        .captures_iter(query.as_str())
        .map(|c| (c.get(0).unwrap().as_str(), true))
        .collect();

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
                    if evt_matches(&log, &evt_tokens) {
                        matched = true;
                        return;
                    }
                });
            };
            if let Some(ca) = calls.get(&hash) {
                ca.iter().for_each(|call| {
                    if call_matches(&call, &call_tokens) {
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
