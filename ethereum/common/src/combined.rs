use crate::calls::*;
use crate::events::*;
use crate::pb::sf::substreams::ethereum::v1::{
    Call, Event, EventsAndCalls, Transaction, Transactions,
};
use crate::pb::sf::substreams::v1::Clock;
use anyhow::Ok;
use buffa::view::{LazyMessageView, MessageView};
use substreams::errors::Error;
use substreams::pb::sf::substreams::index::v1::Keys;
use substreams::Hex;
use substreams_ethereum::pb::eth::v2::BlockLazyView;

#[substreams::handlers::map]
fn index_events_and_calls(block: &BlockLazyView<'_>) -> Result<Keys, Error> {
    let mut keys = Keys::default();

    for log in block.logs() {
        keys.keys.extend(evt_keys(&log));
    }

    for call in block.calls() {
        keys.keys.extend(call_keys(&call));
    }

    Ok(keys)
}

#[substreams::handlers::map]
fn filtered_events_and_calls(
    query: String,
    block: &BlockLazyView<'_>,
) -> Result<EventsAndCalls, Error> {
    let matcher = substreams::sqe::expr_matcher(&query);

    let mut events = Vec::new();
    let mut calls = Vec::new();

    for trace in block.transactions() {
        if let Some(receipt) = trace.receipt() {
            for log in receipt.logs.iter() {
                let log = log?;
                let keys = evt_keys(&log);
                let keys = keys.iter().map(|k| k.as_str()).collect::<Vec<&str>>();
                if matcher.matches_keys(&keys) {
                    events.push(Event {
                        tx_hash: Hex::encode(&trace.hash),
                        log: log.to_owned_message()?.into(),
                    });
                }
            }
        }

        for call in trace.calls.iter() {
            let call = call?;
            let keys = call_keys(&call);
            let keys = keys.iter().map(|k| k.as_str()).collect::<Vec<&str>>();
            if matcher.matches_keys(&keys) {
                calls.push(Call {
                    tx_hash: Hex::encode(&trace.hash),
                    call: call.to_owned_message()?.into(),
                });
            }
        }
    }

    let timestamp = match block.header.get()? {
        Some(header) => header.timestamp.to_owned_message()?.into(),
        None => Default::default(),
    };

    Ok(EventsAndCalls {
        events,
        calls,
        clock: Clock {
            timestamp,
            id: Hex::encode(&block.hash),
            number: block.number,
        }
        .into(),
    })
}

#[substreams::handlers::map]
fn filtered_transactions(
    query: String,
    block: &BlockLazyView<'_>,
) -> Result<Transactions, Error> {
    let matcher = substreams::sqe::expr_matcher(&query);

    let mut filtered: Vec<Transaction> = Vec::new();
    for trace in block.transactions() {
        let mut matched = false;

        if let Some(receipt) = trace.receipt() {
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
        Some(header) => header.timestamp.to_owned_message()?.into(),
        None => Default::default(),
    };
    let clock = Clock {
        timestamp,
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
    fn test_index_events_and_calls_carries_both_key_families() {
        // Given
        let bytes =
            testing::read_block_bytes("./src/testdata/ethereum_mainnet_10500500.binpb.base64");
        let block = BlockLazyView::decode_lazy(&bytes).expect("Not able to decode Block");

        // When
        let keys = substreams::testing::map!(index_events_and_calls(&block))
            .expect("Failed to execute function");

        // Expect: the event keys come first, then the call keys, and every key
        // belongs to one of the five namespaces this module documents.
        let events = substreams::testing::map!(crate::events::index_events(&block))
            .expect("Failed to execute function");
        assert_eq!(keys.keys[..events.keys.len()], events.keys[..]);

        assert!(keys.keys[events.keys.len()..].iter().all(|k| {
            k.starts_with("call_from:0x")
                || k.starts_with("call_to:0x")
                || k.starts_with("call_method:0x")
        }));
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

        // `tx_hash` is `Hex::encode`d, so it carries no `0x` prefix. Comparing
        // against a prefixed literal here matched nothing and left every
        // assertion below unreachable.
        let matched = result
            .transactions
            .into_iter()
            .filter(|t| {
                t.tx_hash == "1fa0d8efe5b3eececcb77df26075312f55355ce924d9a7f39362defb5d8fc424"
            })
            .count();

        assert_eq!(matched, 1, "the queried transaction must be in the output");
    }
    /// `filtered_events_and_calls` is the two single-family filters run over the same
    /// block with one query, so each side matches what that family's filter returns.
    #[test]
    fn test_filtered_events_and_calls() {
        let bytes =
            testing::read_block_bytes("./src/testdata/ethereum_mainnet_10500500.binpb.base64");
        let block = BlockLazyView::decode_lazy(&bytes).expect("Not able to decode Block");

        let query = "evt_addr:0x5acc84a3e955bdd76467d3348077d003f00ffb97 || call_from:0x5acc84a3e955bdd76467d3348077d003f00ffb97".to_string();
        let both = substreams::testing::map!(filtered_events_and_calls(query.clone(), &block))
            .expect("Failed to execute");

        let events = substreams::testing::map!(crate::events::filtered_events(
            query.clone(),
            &block
        ))
        .expect("Failed to execute");
        let calls =
            substreams::testing::map!(crate::calls::filtered_calls(query, &block))
                .expect("Failed to execute");

        assert!(!both.events.is_empty());
        assert!(!both.calls.is_empty());
        assert_eq!(both.events.len(), events.events.len());
        assert_eq!(both.calls.len(), calls.calls.len());
        assert_eq!(both.clock.number, 10500500);
    }

}
