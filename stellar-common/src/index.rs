use std::collections::HashSet;

use stellar_xdr::Transaction;
use substreams::pb::sf::substreams::index::v1::Keys;
use serde_json::Value;

use crate::{
    pb::sf::substreams::stellar::r#type::v1::{Event, Events, Transactions},
    utils,
};

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
        keys: keys.into_iter().collect(),
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

    return keys;
}

#[substreams::handlers::map]
fn index_events(events: Events) -> Result<Keys, substreams::errors::Error> {
    let mut keys: HashSet<String> = HashSet::new();

    for event in events.events {
        keys.extend(event_keys(&event));
    }

    Ok(Keys {
        keys: keys.into_iter().collect(),
    })
}

pub fn event_keys(event: &Event) -> Vec<String> {
    let mut keys = Vec::new();

    keys.push(format!("type:{}", event.r#type));

    if let Some(contract_id) = &event.contract_id {
        keys.push(format!("contract_id:{}", contract_id));
    }

    for topic in &event.topics {
        if let Ok(parsed) = serde_json::from_str::<Value>(topic) {
            if let Some(obj) = parsed.as_object() {
                for (key, value) in obj {
                    if let Some(string_value) = value.as_str() {
                        keys.push(format!("topic:{}:{}", key, string_value));
                    }
                }
            }
        }
    }

    keys
}
