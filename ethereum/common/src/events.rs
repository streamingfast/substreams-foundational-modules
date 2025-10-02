use crate::pb::sf::substreams::ethereum::v1::{Event, Events};
use crate::pb::sf::substreams::v1::Clock;
use anyhow::Ok;
use substreams::errors::Error;
use substreams::pb::sf::substreams::index::v1::Keys;
use substreams::Hex;
use substreams_ethereum::pb::eth::v2::Block;

#[substreams::handlers::map]
fn all_events(blk: Block) -> Result<Events, Error> {
    _all_events(blk)
}

/// _all_events is equal to [all_events] but exists only for unit testing purposes.
pub fn _all_events(blk: Block) -> Result<Events, Error> {
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
    let mut keys = Keys::default();

    events.events.into_iter().for_each(|e| {
        if let Some(log) = e.log {
            evt_keys(&log).into_iter().for_each(|k| {
                keys.keys.push(k);
            });
        }
    });

    Ok(keys)
}

#[substreams::handlers::map]
fn filtered_events(query: String, events: Events) -> Result<Events, Error> {
    _filtered_events(query, events)
}

/// _filtered_events is equal to [filtered_events] but exists only for unit testing purposes.
fn _filtered_events(query: String, mut events: Events) -> Result<Events, Error> {
    let matcher: substreams::ExprMatcher<'_> = substreams::expr_matcher(&query);

    events.events.retain(|event| {
        let keys = evt_keys(event.log.as_ref().unwrap());
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
        let result = _filtered_events(
            "evt_addr:0x5acc84a3e955bdd76467d3348077d003f00ffb97".to_owned(),
            _all_events(block).unwrap(),
        )
        .expect("Failed to execute function");

        // Expect
        assert!(result.events.len() > 0);
        result.events.iter().for_each(|e| {
            let address: &Vec<u8> = &e.log.as_ref().unwrap().address;

            assert_eq!(
                Hex::encode(address),
                "5acc84a3e955bdd76467d3348077d003f00ffb97"
            );
        });
    }
}
