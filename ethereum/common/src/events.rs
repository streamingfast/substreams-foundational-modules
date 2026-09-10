use crate::pb::sf::substreams::ethereum::v1::{Event, Events};
use crate::pb::sf::substreams::v1::Clock;
use anyhow::Ok;
use substreams::errors::Error;
use substreams::pb::sf::substreams::index::v1::Keys;
use substreams::Hex;
use substreams_ethereum::pb::eth::v2::{Block, TransactionTraceStatus};

#[substreams::handlers::map]
fn all_events(blk: Block) -> Result<Events, Error> {
    let clock = Clock {
        timestamp: blk.header.timestamp.clone(),
        id: Hex::encode(&blk.hash),
        number: blk.number,
    };

    let events: Vec<Event> = blk
        .transaction_traces
        .into_iter()
        .filter(|tx| tx.status == TransactionTraceStatus::Succeeded)
        .map(|tx| (tx.receipt.into_option().unwrap_or_default().logs, tx.hash))
        .flat_map(|(log, hash)| {
            log.into_iter().map(move |l| Event {
                tx_hash: Hex::encode(&hash),
                log: l.into(),
            })
        })
        .collect();

    Ok(Events {
        events: events,
        clock: clock.into(),
    })
}

#[substreams::handlers::map]
fn index_events(events: Events) -> Result<Keys, Error> {
    let mut keys = Keys::default();

    events.events.into_iter().for_each(|e| {
        if let Some(log) = e.log.as_option() {
            evt_keys(&log).into_iter().for_each(|k| {
                keys.keys.push(k);
            });
        }
    });

    Ok(keys)
}

#[substreams::handlers::map]
fn filtered_events(query: String, mut events: Events) -> Result<Events, Error> {
    let matcher = substreams::sqe::expr_matcher(&query);

    events.events.retain(|event| {
        let Some(log) = event.log.as_option() else {
            return false;
        };
        let keys = evt_keys(log);
        let keys = keys.iter().map(|k| k.as_str()).collect::<Vec<&str>>();

        matcher.matches_keys(&keys)
    });

    Ok(events)
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

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_filtered_events() {
        // Given
        let block: Block =
            testing::read_block("./src/testdata/ethereum_mainnet_10500500.binpb.base64");

        // When
        let result = substreams::testing::map!(filtered_events(
            "evt_addr:0x5acc84a3e955bdd76467d3348077d003f00ffb97".to_owned(),
            substreams::testing::map!(all_events(block)).unwrap(),
        ))
        .expect("Failed to execute function");

        // Expect
        assert!(result.events.len() > 0);
        result.events.iter().for_each(|e| {
            let address: &Vec<u8> = &e.log.address;

            assert_eq!(
                Hex::encode(address),
                "5acc84a3e955bdd76467d3348077d003f00ffb97"
            );
        });
    }
}
