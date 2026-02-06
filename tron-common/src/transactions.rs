use crate::{
    index,
    pb::sf::{
        substreams::{tron::v1::Transactions, v1::Clock},
        tron::r#type::v1::{Block, Transaction},
    },
    utils,
};

#[substreams::handlers::map]
fn map_transactions(clock: Clock, block: Block) -> Result<Transactions, substreams::errors::Error> {
    let transactions: Vec<Transaction> = block
        .transactions
        .into_iter()
        .filter(|tx| !utils::transaction_failed(tx.code))
        .collect();

    Ok(Transactions {
        transactions,
        clock: Some(clock),
    })
}

#[substreams::handlers::map]
fn filtered_transactions(
    query: String,
    mut transactions: Transactions,
) -> Result<Transactions, substreams::errors::Error> {
    let matcher = substreams::sqe::expr_matcher(&query);

    transactions
        .transactions
        .retain(|transaction| matcher.matches_keys(&index::transaction_keys(transaction)));

    Ok(transactions)
}
