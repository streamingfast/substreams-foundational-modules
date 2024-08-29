use crate::pb::sf::substreams::ethereum::v1::{Call, Calls};
use crate::pb::sf::substreams::v1::Clock;
use anyhow::Ok;
use substreams::errors::Error;
use substreams::matches_keys_in_parsed_expr;
use substreams::pb::sf::substreams::index::v1::Keys;
use substreams::Hex;
use substreams_ethereum::pb::eth::v2::Block;

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

#[substreams::handlers::map]
fn index_calls(calls: Calls) -> Result<Keys, Error> {
    let mut keys = Keys::default();

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
fn filtered_calls(query: String, mut calls: Calls) -> Result<Calls, Error> {
    let matcher = substreams::expr_matcher(&query);

    calls.calls.retain(|call| {
        let keys = call_keys(call.call.as_ref().unwrap());
        let keys = keys.iter().map(|k| k.as_str()).collect::<Vec<&str>>();

        matcher.matches_keys(&keys)
    });

    Ok(calls)
}

pub fn call_matches(
    call: &substreams_ethereum::pb::eth::v2::Call,
    query: &str,
) -> Result<bool, Error> {
    matches_keys_in_parsed_expr(&call_keys(call), query)
}

pub fn call_keys(call: &substreams_ethereum::pb::eth::v2::Call) -> Vec<String> {
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
