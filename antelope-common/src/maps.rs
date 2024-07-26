use std::collections::HashSet;

use substreams::{errors::Error, matches_keys_in_parsed_expr};
use substreams_antelope::{
    pb::{ActionTrace, ActionTraces, TransactionTraces},
    Block,
};

use crate::index::{action_keys, action_keys_extra};

#[substreams::handlers::map]
fn all_transactions(block: Block) -> Result<TransactionTraces, Error> {
    Ok(TransactionTraces {
        transaction_traces: block.into_transaction_traces().collect(),
    })
}

#[substreams::handlers::map]
fn all_actions(transactions: TransactionTraces) -> Result<ActionTraces, Error> {
    let action_traces = transactions
        .transaction_traces
        .into_iter()
        .flat_map(|trx| trx.action_traces)
        .collect::<Vec<_>>();

    Ok(ActionTraces { action_traces })
}

#[substreams::handlers::map]
fn filtered_actions(query: String, actions: ActionTraces) -> Result<ActionTraces, Error> {
    filter_actions(query, actions, action_keys)
}

#[substreams::handlers::map]
fn filtered_actions_extra(query: String, actions: ActionTraces) -> Result<ActionTraces, Error> {
    filter_actions(query, actions, action_keys_extra)
}

#[substreams::handlers::map]
fn filtered_transactions(
    query: String,
    transactions: TransactionTraces,
) -> Result<TransactionTraces, Error> {
    filter_transactions(query, transactions, action_keys)
}

#[substreams::handlers::map]
fn filtered_transactions_extra(
    query: String,
    transactions: TransactionTraces,
) -> Result<TransactionTraces, Error> {
    filter_transactions(query, transactions, action_keys_extra)
}

fn filter_actions<F>(
    query: String,
    actions: ActionTraces,
    key_extractor: F,
) -> Result<ActionTraces, Error>
where
    F: Fn(&ActionTrace) -> Vec<String>,
{
    let action_traces = actions
        .action_traces
        .into_iter()
        .filter(|action| {
            let keys = key_extractor(action);

            // will panic if the query is invalid
            matches_keys_in_parsed_expr(&keys, &query).unwrap()
        })
        .collect();

    Ok(ActionTraces { action_traces })
}

fn filter_transactions<F>(
    query: String,
    transactions: TransactionTraces,
    key_extractor: F,
) -> Result<TransactionTraces, Error>
where
    F: Fn(&ActionTrace) -> Vec<String>,
{
    let transaction_traces = transactions
        .transaction_traces
        .into_iter()
        .filter(|trx| {
            let keys = trx
                .action_traces
                .iter()
                .flat_map(&key_extractor)
                .collect::<HashSet<_>>()
                .into_iter()
                .collect::<Vec<_>>();

            // will panic if the query is invalid
            matches_keys_in_parsed_expr(&keys, &query).unwrap()
        })
        .collect();

    Ok(TransactionTraces { transaction_traces })
}
