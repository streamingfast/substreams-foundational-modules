use crate::pb::sol::instructions::v1::Instruction;
use crate::pb::sol::instructions::v1::Instructions;
use crate::pb::sol::transactions::v1::Transactions;
use substreams::pb::sf::substreams::index::v1::Keys;
use substreams_solana::base58;
use substreams_solana::pb::sf::solana::r#type::v1::ConfirmedTransaction;

#[substreams::handlers::map]
fn all_instructions_without_votes(transactions: Transactions) -> Result<Instructions, substreams::errors::Error> {
    let instructions = get_instructions_from_transactions(&transactions.transactions);

    Ok(Instructions { instructions })
}

#[substreams::handlers::map]
fn index_instructions_without_votes(instructions: Instructions) -> Result<Keys, substreams::errors::Error> {
    let keys: Vec<String> = instructions
        .instructions
        .into_iter()
        .map(|inst| format!("program:{}", inst.program_id))
        .collect();

    Ok(Keys { keys })
}

pub fn get_instructions_from_transactions(transactions: &Vec<ConfirmedTransaction>) -> Vec<Instruction> {
    let mut processed_instructions: Vec<Instruction> = Vec::new();

    for confirmed_transaction in transactions.iter() {
        // FIXME: If the progam would be resolved right away, we could remove this need entirely here
        let acct_keys: Vec<&[u8]> = confirmed_transaction
            .resolved_accounts()
            .iter()
            .map(|k| k.as_slice())
            .collect();

        if let Some(transaction) = &confirmed_transaction.transaction {
            for view in confirmed_transaction.all_instructions().into_iter() {
                processed_instructions.push(Instruction {
                    program_id: bs58::encode(acct_keys[view.program_id_index() as usize]).into_string(),
                    data: view.data().clone(),
                    accounts: view.resolved_accounts.iter().map(base58::encode).collect(),
                    tx_hash: transaction.id(),
                });
            }
        }
    }

    processed_instructions
}
