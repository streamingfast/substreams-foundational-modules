use std::collections::BTreeSet;

use crate::actions::action_keys;
use substreams::matches_keys_in_parsed_expr;
use substreams_antelope::{
    pb::{ActionTraces, TransactionTraces},
    Block,
};

#[substreams::handlers::map]
fn all_transactions(block: Block) -> Result<TransactionTraces, substreams::errors::Error> {
    Ok(TransactionTraces {
        transaction_traces: block.into_transaction_traces().collect(),
    })
}

#[substreams::handlers::map]
fn all_actions(transactions: TransactionTraces) -> Result<ActionTraces, substreams::errors::Error> {
    let action_traces = transactions
        .transaction_traces
        .into_iter()
        .flat_map(|trx| trx.action_traces)
        .collect::<Vec<_>>();

    Ok(ActionTraces { action_traces })
}

#[substreams::handlers::map]
fn filtered_actions(
    query: String,
    actions: ActionTraces,
) -> Result<ActionTraces, substreams::errors::Error> {
    let action_traces = actions
        .action_traces
        .into_iter()
        .filter(|action| {
            let keys = action_keys(action)
                .into_iter()
                .collect::<BTreeSet<_>>()
                .into_iter()
                .collect::<Vec<_>>();

            // will panic if the query is invalid
            matches_keys_in_parsed_expr(&keys, &query).unwrap()
        })
        .collect();

    Ok(ActionTraces { action_traces })
}

#[substreams::handlers::map]
fn filtered_transactions(
    query: String,
    transactions: TransactionTraces,
) -> Result<TransactionTraces, substreams::errors::Error> {
    let transaction_traces = transactions
        .transaction_traces
        .into_iter()
        .filter(|trx| {
            let keys = trx
                .action_traces
                .iter()
                .flat_map(action_keys)
                .collect::<BTreeSet<_>>()
                .into_iter()
                .collect::<Vec<_>>();

            // will panic if the query is invalid
            matches_keys_in_parsed_expr(&keys, &query).unwrap()
        })
        .collect();

    Ok(TransactionTraces { transaction_traces })
}
