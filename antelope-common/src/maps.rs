use std::collections::BTreeSet;

use crate::{
    actions::action_keys,
    pb::sf::substreams::antelope::v1::{Action, Actions, Transaction, Transactions},
};
use substreams::matches_keys_in_parsed_expr;
use substreams_antelope::Block;

#[substreams::handlers::map]
fn all_transactions(block: Block) -> Result<Transactions, substreams::errors::Error> {
    let all = block
        .into_transaction_traces()
        .map(|trx| Transaction {
            tx_hash: trx.id.clone(),
            trace: Some(trx),
        })
        .collect();

    Ok(Transactions { transactions: all })
}

#[substreams::handlers::map]
fn all_actions(transactions: Transactions) -> Result<Actions, substreams::errors::Error> {
    let all = transactions
        .transactions
        .into_iter()
        .flat_map(|trx| {
            trx.trace
                .unwrap()
                .action_traces
                .into_iter()
                .map(move |action| Action {
                    tx_hash: trx.tx_hash.clone(),
                    trace: Some(action),
                })
        })
        .collect();

    Ok(Actions { actions: all })
}

#[substreams::handlers::map]
fn filtered_actions(query: String, actions: Actions) -> Result<Actions, substreams::errors::Error> {
    let filtered = actions
        .actions
        .into_iter()
        .filter(|action| {
            let keys = action_keys(action.trace.as_ref().unwrap())
                .into_iter()
                .collect::<BTreeSet<_>>()
                .into_iter()
                .collect::<Vec<_>>();

            // will panic if the query is invalid
            matches_keys_in_parsed_expr(&keys, &query).unwrap()
        })
        .collect();

    Ok(Actions { actions: filtered })
}

#[substreams::handlers::map]
fn filtered_transactions(
    query: String,
    transactions: Transactions,
) -> Result<Transactions, substreams::errors::Error> {
    let filtered = transactions
        .transactions
        .into_iter()
        .filter(|trx| {
            let keys = trx
                .trace
                .as_ref()
                .unwrap()
                .action_traces
                .iter()
                .flat_map(|action| action_keys(&action))
                .collect::<BTreeSet<_>>()
                .into_iter()
                .collect::<Vec<_>>();

            // will panic if the query is invalid
            matches_keys_in_parsed_expr(&keys, &query).unwrap()
        })
        .collect();

    Ok(Transactions {
        transactions: filtered,
    })
}
