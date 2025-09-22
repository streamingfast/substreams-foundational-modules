use substreams::pb::sf::substreams::index::v1::Keys;
use substreams_solana::pb::sf::solana::r#type::v1::Block;

use crate::keys::transaction_program_and_account_keys;

#[substreams::handlers::map]
fn program_ids_without_votes(block: Block) -> Result<Keys, substreams::errors::Error> {
    let keys: Vec<String> = block
        .walk_instructions()
        .map(|inst| format!("program:{}", inst.program_id()))
        .collect();

    Ok(Keys { keys })
}

#[substreams::handlers::map]
fn program_ids_and_accounts_without_votes(block: Block) -> Result<Keys, substreams::errors::Error> {
    Ok(Keys {
        keys: block
            .transactions()
            .flat_map(transaction_program_and_account_keys)
            .collect(),
    })
}
