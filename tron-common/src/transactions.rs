use buffa::view::LazyMessageView;

use crate::{
    index,
    pb::sf::{
        substreams::{tron::v1::Transactions, v1::Clock},
        tron::r#type::v1::{BlockLazyView, Transaction},
    },
    utils,
};

#[substreams::handlers::map]
fn map_transactions(
    clock: Clock,
    block: &BlockLazyView<'_>,
) -> Result<Transactions, substreams::errors::Error> {
    let mut transactions: Vec<Transaction> = Vec::new();

    for transaction in block.transactions.iter() {
        let transaction = transaction?;

        if utils::transaction_failed(transaction.code.to_i32()) {
            continue;
        }

        transactions.push(transaction.to_owned_message()?);
    }

    Ok(Transactions {
        transactions,
        clock: clock.into(),
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
