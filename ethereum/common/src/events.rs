use crate::pb::sf::substreams::ethereum::v1::{Event, Events};
use crate::pb::sf::substreams::v1::Clock;
use anyhow::Ok;
use buffa::view::{LazyMessageView, MessageView};
use substreams::errors::Error;
use substreams::pb::sf::substreams::index::v1::Keys;
use substreams::Hex;
use substreams_ethereum::pb::eth::v2::BlockLazyView;

#[substreams::handlers::map]
fn index_events(block: &BlockLazyView<'_>) -> Result<Keys, Error> {
    let mut keys = Keys::default();

    for log in block.logs() {
        keys.keys.extend(evt_keys(&log));
    }

    Ok(keys)
}

#[substreams::handlers::map]
fn filtered_events(query: String, block: &BlockLazyView<'_>) -> Result<Events, Error> {
    let matcher = substreams::sqe::expr_matcher(&query);

    let mut events = Vec::new();
    for trace in block.transactions() {
        let Some(receipt) = trace.receipt() else {
            continue;
        };

        for log in receipt.logs.iter() {
            let log = log?;
            let keys = evt_keys(&log);
            let keys = keys.iter().map(|k| k.as_str()).collect::<Vec<&str>>();
            if !matcher.matches_keys(&keys) {
                continue;
            }

            events.push(Event {
                tx_hash: Hex::encode(&trace.hash),
                log: log.to_owned_message()?.into(),
            });
        }
    }

    let timestamp = match block.header.get()? {
        Some(header) => header.timestamp.to_owned_message()?.into(),
        None => Default::default(),
    };

    Ok(Events {
        events,
        clock: Clock {
            timestamp,
            id: Hex::encode(&block.hash),
            number: block.number,
        }
        .into(),
    })
}

/// The log fields the index keys are built from, implemented for both the owned
/// `Log` and buffa's `LogLazyView`.
pub trait EvtKeyed {
    fn first_topic(&self) -> Option<&[u8]>;
    fn key_address(&self) -> &[u8];
}

impl EvtKeyed for substreams_ethereum::pb::eth::v2::Log {
    fn first_topic(&self) -> Option<&[u8]> {
        self.topics.get(0).map(|t| t.as_ref())
    }

    fn key_address(&self) -> &[u8] {
        &self.address
    }
}

impl EvtKeyed for substreams_ethereum::pb::eth::v2::LogLazyView<'_> {
    fn first_topic(&self) -> Option<&[u8]> {
        self.topics.get(0).map(|t| &**t)
    }

    fn key_address(&self) -> &[u8] {
        self.address
    }
}

impl<T: EvtKeyed + ?Sized> EvtKeyed for &T {
    fn first_topic(&self) -> Option<&[u8]> {
        (**self).first_topic()
    }

    fn key_address(&self) -> &[u8] {
        (**self).key_address()
    }
}

pub fn evt_keys<L: EvtKeyed + ?Sized>(log: &L) -> Vec<String> {
    let mut keys = Vec::new();

    if let Some(topic) = log.first_topic() {
        let k_log_sign = format!("evt_sig:0x{}", Hex::encode(topic));
        keys.push(k_log_sign);
    }

    let k_log_address = format!("evt_addr:0x{}", Hex::encode(log.key_address()));
    keys.push(k_log_address);

    keys
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use buffa::view::LazyMessageView;

    #[test]
    fn test_index_events() {
        // Given
        let bytes = testing::read_block_bytes("./src/testdata/ethereum_mainnet_10500500.binpb.base64");
        let block = BlockLazyView::decode_lazy(&bytes).expect("Not able to decode Block");

        // When
        let keys = substreams::testing::map!(index_events(&block))
            .expect("Failed to execute function");

        // Expect
        assert!(keys.keys.len() > 0);
        assert!(keys
            .keys
            .iter()
            .all(|k| k.starts_with("evt_sig:0x") || k.starts_with("evt_addr:0x")));
        assert!(keys
            .keys
            .iter()
            .any(|k| k == "evt_addr:0x5acc84a3e955bdd76467d3348077d003f00ffb97"));
    }

    /// A query on one event address keeps only that address's logs, and a query that
    /// cannot match keeps none.
    #[test]
    fn test_filtered_events() {
        let bytes =
            testing::read_block_bytes("./src/testdata/ethereum_mainnet_10500500.binpb.base64");
        let block = BlockLazyView::decode_lazy(&bytes).expect("Not able to decode Block");

        let query = "evt_addr:0x5acc84a3e955bdd76467d3348077d003f00ffb97".to_string();
        let events =
            substreams::testing::map!(filtered_events(query, &block)).expect("Failed to execute");

        assert!(!events.events.is_empty());
        assert!(events.events.iter().all(|e| {
            Hex::encode(&e.log.address) == "5acc84a3e955bdd76467d3348077d003f00ffb97"
        }));
        assert_eq!(events.clock.number, 10500500);

        let none = substreams::testing::map!(filtered_events(
            "evt_addr:0x0000000000000000000000000000000000000000".to_string(),
            &block
        ))
        .expect("Failed to execute");
        assert!(none.events.is_empty());
    }
}
