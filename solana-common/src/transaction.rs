use std::collections::HashMap;
use crate::instruction::get_instructions_from_transactions;
use crate::pb::sol::transactions::v1::Transactions;
use substreams::matches_keys_in_parsed_expr;
use substreams_solana::pb::sf::solana::r#type::v1::Block;
use substreams_solana::pb::sf::solana::r#type::v1::ConfirmedTransaction;

#[substreams::handlers::map]
fn all_transactions_without_votes(blk: Block) -> Result<Transactions, substreams::errors::Error> {
    let transactions: Vec<ConfirmedTransaction> = blk.transactions.into_iter().collect();

    Ok(Transactions { transactions })
}

#[substreams::handlers::map]
fn filtered_txs_by_instructions_without_votes(query: String, transactions: Transactions) -> Result<Transactions, substreams::errors::Error> {

    let instructions = get_instructions_from_transactions(&transactions.transactions);
    let matching_trx_hashes = instructions
            .into_iter()
            .filter(|inst| {
                let mut keys = Vec::new();
                keys.push(format!("program:{}", inst.program_id));

                return matches_keys_in_parsed_expr(&keys, &query).expect("matching events from query")
            }).map(|inst| (inst.tx_hash, true))
            .collect::<HashMap<String, bool>>();

    let filtered_transations: Vec<ConfirmedTransaction> = transactions.transactions
            .into_iter()
            .filter(|trans| {
                let hash = bs58::encode(&trans.transaction.as_ref().unwrap().signatures[0]).into_string();

                return matching_trx_hashes.contains_key(&hash);
            }).collect();

    Ok(Transactions {
        transactions: filtered_transations
    })
}

