use substreams::pb::sf::substreams::index::v1::Keys;
use substreams_solana::block_view::lazy::LazyTransaction;
use substreams_solana::pb::sf::solana::r#type::v1::BlockLazyView;

use crate::keys::lazy_transaction_program_and_account_keys_vec;

#[substreams::handlers::map]
fn program_ids_without_votes(block: &BlockLazyView<'_>) -> Result<Keys, substreams::errors::Error> {
    let mut keys = Vec::new();

    for trx in block.transactions() {
        let trx = LazyTransaction::new(&trx)?;
        for inst in trx.walk_instructions()? {
            keys.push(format!("program:{}", inst.program_id()));
        }
    }

    Ok(Keys { keys })
}

#[substreams::handlers::map]
fn program_ids_and_accounts_without_votes(
    block: &BlockLazyView<'_>,
) -> Result<Keys, substreams::errors::Error> {
    let mut keys = Vec::new();

    for trx in block.transactions() {
        let trx = LazyTransaction::new(&trx)?;
        keys.extend(lazy_transaction_program_and_account_keys_vec(&trx));
    }

    Ok(Keys { keys })
}
