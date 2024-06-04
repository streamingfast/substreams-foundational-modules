use std::collections::HashSet;

use substreams::pb::sf::substreams::index::v1::Keys;
use substreams_antelope::pb::ActionTraces;

use crate::actions::action_keys;

#[substreams::handlers::map]
fn index_actions(actions: ActionTraces) -> Result<Keys, substreams::errors::Error> {
    let keys = actions
        .action_traces
        .into_iter()
        .flat_map(|action| action_keys(&action))
        .collect::<HashSet<_>>()
        .into_iter()
        .collect();

    Ok(Keys { keys })
}
