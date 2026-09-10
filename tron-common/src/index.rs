use crate::pb::sf::substreams::tron::v1::Transactions;
use crate::pb::sf::tron::r#type::v1::Transaction;
use crate::utils::{extract_from_address, extract_to_address, tron_address_to_base58};
use buffa::Enumeration;
use std::collections::HashSet;
use substreams::pb::sf::substreams::index::v1::Keys;

#[substreams::handlers::map]
fn index_transactions(transactions: Transactions) -> Result<Keys, substreams::errors::Error> {
    let mut keys = HashSet::new();

    for transaction in transactions.transactions {
        for key in transaction_keys(&transaction) {
            keys.insert(key);
        }
    }

    Ok(Keys {
        keys: keys.into_iter().collect(),
    })
}

pub fn transaction_keys(transaction: &Transaction) -> Vec<String> {
    let mut keys = Vec::new();

    for contract in &transaction.contracts {
        keys.push(format!(
            "contract_type:{}",
            contract
                .r#type
                .as_known()
                .map(|t| t.proto_name())
                .unwrap_or("Unknown")
        ));

        if let Some(parameter) = contract.parameter.as_option() {
            if let Some(owner_bytes) = extract_from_address(contract.r#type.to_i32(), parameter) {
                let tron_address = tron_address_to_base58(&owner_bytes);
                keys.push(format!("from:{}", tron_address));
            }
        }
        if let Some(parameter) = contract.parameter.as_option() {
            if let Some(owner_bytes) = extract_to_address(contract.r#type.to_i32(), parameter) {
                let tron_address = tron_address_to_base58(&owner_bytes);
                keys.push(format!("to:{}", tron_address));
            }
        }
    }

    if let Some(info) = transaction.info.as_option() {
        if !info.contract_address.is_empty() {
            let tron_address = tron_address_to_base58(&info.contract_address);
            keys.push(format!("contract_address:{}", tron_address));
        }
    }

    keys
}
