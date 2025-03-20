use std::collections::HashSet;

use stellar_xdr::curr::Transaction;
use substreams::pb::sf::substreams::index::v1::Keys;

use crate::{pb::sf::substreams::stellar::r#type::v1::Transactions, utils};

#[substreams::handlers::map]
fn index_transactions(transactions: Transactions) -> Result<Keys, substreams::errors::Error> {
    let keys: HashSet<String> = transactions
        .transactions
        .into_iter()
        .filter_map(|transaction| {
            let trx =
                match utils::decode_transaction(&transaction.result_xdr, &transaction.envelope_xdr)
                {
                    Ok(trx) => trx,
                    Err(_) => return None,
                };

            Some(transaction_keys(trx))
        })
        .flatten()
        .collect();

    Ok(Keys {
        keys: keys.into_iter().collect()
    })
}

pub fn transaction_keys(trx: Transaction) -> Vec<String> {
    let mut keys = vec![format!(
        "source_account:{}",
        trx.source_account.account_id().to_string()
    )];

    for operation in trx.operations.iter() {
        if let Some(source_account) = &operation.source_account {
            keys.push(format!(
                "source_account:{}",
                source_account.clone().account_id().to_string()
            ));
        }
    }

    return keys
}
