use std::collections::BTreeSet;

use substreams::pb::sf::substreams::index::v1::Keys;
use substreams_antelope::pb::Block;

use crate::actions::action_keys;

#[substreams::handlers::map]
fn index_actions(block: Block) -> Result<Keys, substreams::errors::Error> {
    let keys = block
        .into_transaction_traces()
        .flat_map(|trx| {
            trx.action_traces
                .into_iter()
                .flat_map(|action| action_keys(&action))
        })
        .collect::<BTreeSet<_>>();

    Ok(Keys {
        keys: keys.into_iter().collect(),
    })
}
