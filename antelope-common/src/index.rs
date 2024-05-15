use std::collections::HashSet;

use substreams::{errors::Error, log};
use substreams_antelope::pb::Block;

use crate::pb::sf::substreams::index::v1::Keys;

#[substreams::handlers::map]
fn index_stuff(block: Block) -> Result<Keys, Error> {
    let keys = block
        .into_transaction_traces()
        .flat_map(|trx| {
            trx.action_traces
                .into_iter()
                .flat_map(|_action| vec!["asdf".to_string(), "qwer".to_string()])
        })
        .collect::<HashSet<_>>();

    log::info!("keys: {:?}", keys);
    Ok(Keys {
        keys: keys.into_iter().collect(),
    })
}
