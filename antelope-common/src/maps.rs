use std::collections::BTreeSet;

use crate::{
    actions::action_keys,
    pb::sf::substreams::antelope::v1::{Transaction, Transactions},
};
use substreams::matches_keys_in_parsed_expr;
use substreams_antelope::Block;

#[substreams::handlers::map]
fn filtered_transactions(
    query: String,
    block: Block,
) -> Result<Transactions, substreams::errors::Error> {
    let filtered = block.into_transaction_traces().filter_map(|trx| {
        let keys = trx
            .action_traces
            .iter()
            .flat_map(|action| action_keys(&action))
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect::<Vec<_>>();

        // will panic if the query is invalid
        if !matches_keys_in_parsed_expr(&keys, &query).unwrap() {
            return None;
        }
        Some(Transaction {
            tx_hash: trx.id.clone(),
            trace: Some(trx),
        })
    });

    Ok(Transactions {
        transactions: filtered.collect(),
    })
}
