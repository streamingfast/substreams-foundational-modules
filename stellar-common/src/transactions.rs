use crate::{
    index,
    pb::sf::{
        stellar::r#type::v1::Block,
        substreams::stellar::r#type::v1::{Transaction, Transactions},
    },
    utils,
};

#[substreams::handlers::map]
fn map_transactions(block: Block) -> Result<Transactions, substreams::errors::Error> {
    let transactions: Vec<Transaction> = block
        .transactions
        .into_iter()
        .filter(|transaction| !utils::transaction_failed(transaction.status))
        .map(|transaction| Transaction {
            hash: transaction.hash,
            status: transaction.status,
            created_at: transaction.created_at,
            application_order: transaction.application_order,
            envelope_xdr: transaction.envelope_xdr,
            result_meta_xdr: transaction.result_meta_xdr,
            result_xdr: transaction.result_xdr,
            block_number: block.number,
        })
        .collect();

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

            let query = substreams::expr_matcher(&query);

            match query.matches_keys(&index::transaction_keys(trx)) {
                true => return Some(transaction),
                false => return None,
            }
        })
        .collect();

    Ok(Transactions { transactions })
}
