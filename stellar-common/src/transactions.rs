use buffa::view::MessageView;

use crate::{
    index,
    pb::sf::{
        stellar::r#type::v1::BlockLazyView,
        substreams::stellar::r#type::v1::{Transaction, Transactions},
    },
    utils,
};

#[substreams::handlers::map]
fn map_transactions(
    block: &BlockLazyView<'_>,
) -> Result<Transactions, substreams::errors::Error> {
    let mut transactions: Vec<Transaction> = Vec::new();

    for transaction in block.transactions.iter() {
        let transaction = transaction?;

        if utils::transaction_failed(transaction.status.to_i32()) {
            continue;
        }

        transactions.push(Transaction {
            hash: transaction.hash.to_vec(),
            status: transaction.status.to_i32(),
            created_at: transaction
                .created_at
                .as_option()
                .map(|t| t.to_owned_message())
                .transpose()?
                .into(),
            application_order: transaction.application_order,
            envelope_xdr: transaction.envelope_xdr.to_vec(),
            result_xdr: transaction.result_xdr.to_vec(),
            block_number: block.number,
        });
    }

    Ok(Transactions { transactions })
}

#[substreams::handlers::map]
fn filtered_transactions(
    query: String,
    transactions: Transactions,
) -> Result<Transactions, substreams::errors::Error> {
    let transactions: Vec<Transaction> = transactions
        .transactions
        .into_iter()
        .filter_map(|transaction| {
            let trx =
                match utils::decode_transaction(&transaction.result_xdr, &transaction.envelope_xdr)
                {
                    Ok(trx) => trx,
                    Err(_) => return None,
                };

            let query = substreams::sqe::expr_matcher(&query);

            match query.matches_keys(&index::transaction_keys(trx)) {
                true => return Some(transaction),
                false => return None,
            }
        })
        .collect();

    Ok(Transactions { transactions })
}
