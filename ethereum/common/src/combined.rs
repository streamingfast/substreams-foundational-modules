use crate::calls::*;
use crate::events::*;
use crate::pb::sf::substreams::ethereum::v1::{Transaction, Transactions};
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
}
