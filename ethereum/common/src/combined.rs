use crate::calls::*;
use crate::events::*;
use crate::pb::sf::substreams::ethereum::v1::{
    Calls, Events, EventsAndCalls, Transaction, Transactions,
};
use crate::pb::sf::substreams::v1::Clock;
use anyhow::Ok;
use buffa::view::{LazyMessageView, MessageView};
use substreams::errors::Error;
use substreams::pb::sf::substreams::index::v1::Keys;
use substreams::Hex;
use substreams_ethereum::pb::eth::v2::TransactionTraceStatus;
use substreams_ethereum::pb::eth::v2::BlockLazyView;

#[substreams::handlers::map]
fn index_events_and_calls(events: Events, calls: Calls) -> Result<Keys, Error> {
    let mut keys = Keys::default();

    events.events.into_iter().for_each(|e| {
        if let Some(log) = e.log.as_option() {
            evt_keys(&log).into_iter().for_each(|k| {
                keys.keys.push(k);
            });
        }
    });

    calls.calls.into_iter().for_each(|call| {
        if let Some(call) = call.call.as_option() {
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
    mut events: Events,
    mut calls: Calls,
) -> Result<EventsAndCalls, Error> {
    let matcher = substreams::sqe::expr_matcher(&query);

    calls.calls.retain(|call| {
        let Some(inner) = call.call.as_option() else {
            return false;
        };
        let keys = call_keys(inner);
        let keys = keys.iter().map(|k| k.as_str()).collect::<Vec<&str>>();

        matcher.matches_keys(&keys)
    });

    events.events.retain(|event| {
        let Some(log) = event.log.as_option() else {
            return false;
        };
        let keys = evt_keys(log);
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
fn filtered_transactions(
    query: String,
    block: &BlockLazyView<'_>,
) -> Result<Transactions, Error> {
    let matcher = substreams::sqe::expr_matcher(&query);

    let mut filtered: Vec<Transaction> = Vec::new();
    for trace in block.try_transactions() {
        let trace = trace?;
        if trace.status.as_known() != Some(TransactionTraceStatus::Succeeded) {
            continue;
        }

        let mut matched = false;

        if let Some(receipt) = trace.receipt()? {
            for log in receipt.logs.iter() {
                let log = log?;
                let keys = evt_keys(&log);
                let keys = keys.iter().map(|k| k.as_str()).collect::<Vec<&str>>();
                if matcher.matches_keys(&keys) {
                    matched = true;
                    break;
                }
            }
        }

        if !matched {
            for call in trace.calls.iter() {
                let call = call?;
                let keys = call_keys(&call);
                let keys = keys.iter().map(|k| k.as_str()).collect::<Vec<&str>>();
                if matcher.matches_keys(&keys) {
                    matched = true;
                    break;
                }
            }
        }

        if matched {
            filtered.push(Transaction {
                tx_hash: Hex::encode(&trace.hash),
                trace: trace.to_owned_message()?.into(),
            });
        }
    }

    let timestamp = match block.header.get()? {
        Some(header) => header.timestamp.as_option().map(|t| t.to_owned_message()),
        None => None,
    };
    let clock = Clock {
        timestamp: timestamp.transpose()?.into(),
        id: Hex::encode(&block.hash),
        number: block.number,
    };

    Ok(Transactions {
        transactions: filtered,
        clock: clock.into(),
        detail_level: block.detail_level.to_i32().into(),
    })
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_filtered_events_and_calls() {
        // Given
        let block: substreams_ethereum::pb::eth::v2::Block =
            testing::read_block("./src/testdata/ethereum_mainnet_10500500.binpb.base64");

        // When
        let result = substreams::testing::map!(filtered_events_and_calls(
            "evt_addr:0x6b175474e89094c44da98b954eedeac495271d0f || call_method:0x029b2f34"
                .to_owned(),
            substreams::testing::map!(all_events(block.clone())).unwrap(),
            substreams::testing::map!(all_calls(block)).unwrap(),
        ))
        .expect("Failed to execute function");

        // Expect
        assert!(result.events.len() > 0);
        result.events.iter().for_each(|e| {
            let address: &Vec<u8> = &e.log.address;

            assert_eq!(
                Hex::encode(address),
                "6b175474e89094c44da98b954eedeac495271d0f"
            );
        });

        result.calls.iter().for_each(|c| {
            let input_bytes = &c.call.input;

            assert_eq!(Hex::encode(&input_bytes[..4]), "029b2f34");
        });
    }

    #[test]
    fn test_filtered_transactions() {
        // Given
        let bytes =
            testing::read_block_bytes("./src/testdata/ethereum_mainnet_10500500.binpb.base64");
        let block = BlockLazyView::decode_lazy(&bytes).expect("Not able to decode Block");

        // When
        let result = substreams::testing::map!(filtered_transactions(
            "evt_addr:0x6b175474e89094c44da98b954eedeac495271d0f || call_method:0x029b2f34"
                .to_owned(),
            &block,
        ))
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
                t.trace.logs_with_calls().for_each(|lc| {
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
