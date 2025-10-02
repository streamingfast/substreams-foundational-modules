use crate::calls::*;
use crate::events::*;
use crate::pb::sf::substreams::ethereum::v1::{
    Calls, Events, EventsAndCalls, Transaction, Transactions,
};
use crate::pb::sf::substreams::v1::Clock;
use anyhow::Ok;
use std::collections::HashMap;
use substreams::errors::Error;
use substreams::pb::sf::substreams::index::v1::Keys;
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
    _filtered_events_and_calls(query, events, calls)
}

/// _filtered_events_and_calls is equal to [filtered_events_and_calls] but exists only for unit testing purposes.
fn _filtered_events_and_calls(
    query: String,
    mut events: Events,
    mut calls: Calls,
) -> Result<EventsAndCalls, Error> {
    let matcher: substreams::ExprMatcher<'_> = substreams::expr_matcher(&query);

    calls.calls.retain(|call| {
        let keys = call_keys(call.call.as_ref().unwrap());
        let keys = keys.iter().map(|k| k.as_str()).collect::<Vec<&str>>();

        matcher.matches_keys(&keys)
    });

    events.events.retain(|event| {
        let keys = evt_keys(event.log.as_ref().unwrap());
        let keys = keys.iter().map(|k| k.as_str()).collect::<Vec<&str>>();

        matcher.matches_keys(&keys)
    });

    Ok(EventsAndCalls {
        events: events.events,
        calls: calls.calls,
        clock: calls.clock,
    })
}

#[substreams::handlers::map]
fn filtered_transactions(query: String, block: Block) -> Result<Transactions, Error> {
    _filtered_transactions(query, block)
}

/// _filtered_transactions is equal to [filtered_transactions] but exists only for unit testing purposes.
fn _filtered_transactions(query: String, block: Block) -> Result<Transactions, Error> {
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

    let matcher: substreams::ExprMatcher<'_> = substreams::expr_matcher(&query);

    let filtered: Vec<Transaction> = block
        .transaction_traces
        .iter()
        .filter(|tx| tx.status == 1)
        .filter(|tt| {
            let mut matched = false;
            let hash = Hex::encode(&tt.hash);
            if let Some(ev) = events.get(&hash) {
                ev.iter().for_each(|log| {
                    let keys = evt_keys(log);
                    let keys = keys.iter().map(|k| k.as_str()).collect::<Vec<&str>>();

                    if matcher.matches_keys(&keys) {
                        matched = true;
                        return;
                    }
                });
            };
            if let Some(ca) = calls.get(&hash) {
                ca.iter().for_each(|call| {
                    let keys = call_keys(call);
                    let keys = keys.iter().map(|k| k.as_str()).collect::<Vec<&str>>();

                    if matcher.matches_keys(&keys) {
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

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_filtered_events_and_calls() {
        // Given
        let block: Block =
            testing::read_block("./src/testdata/ethereum_mainnet_10500500.binpb.base64");

        // When
        let result = _filtered_events_and_calls(
            "evt_addr:0x6b175474e89094c44da98b954eedeac495271d0f || call_method:0x029b2f34"
                .to_owned(),
            _all_events(block.clone()).unwrap(),
            _all_calls(block).unwrap(),
        )
        .expect("Failed to execute function");

        // Expect
        assert!(result.events.len() > 0);
        result.events.iter().for_each(|e| {
            let address: &Vec<u8> = &e.log.as_ref().unwrap().address;

            assert_eq!(
                Hex::encode(address),
                "6b175474e89094c44da98b954eedeac495271d0f"
            );
        });

        result.calls.iter().for_each(|c| {
            let input_bytes = &c.call.as_ref().unwrap().input;

            assert_eq!(Hex::encode(&input_bytes[..4]), "029b2f34");
        });
    }

    #[test]
    fn test_filtered_transactions() {
        // Given
        let block: Block =
            testing::read_block("./src/testdata/ethereum_mainnet_10500500.binpb.base64");

        // When
        let result = _filtered_transactions(
            "evt_addr:0x6b175474e89094c44da98b954eedeac495271d0f || call_method:0x029b2f34"
                .to_owned(),
            block,
        )
        .expect("Failed to execute function");

        // Expect
        assert!(result.transactions.len() > 0);
        result
            .transactions
            .into_iter()
            .filter(|t| {
                t.tx_hash == "0x1fa0d8efe5b3eececcb77df26075312f55355ce924d9a7f39362defb5d8fc424"
            })
            .for_each(|t| {
                t.trace.unwrap().logs_with_calls().for_each(|lc| {
                    let input_bytes = &lc.1.as_ref().input;

                    assert_eq!(
                        Hex::encode(&lc.0.address),
                        "0x6b175474e89094c44da98b954eedeac495271d0f"
                    );
                    assert_eq!(Hex::encode(&input_bytes[..4]), "029b2f34");
                });
            });
    }
}
