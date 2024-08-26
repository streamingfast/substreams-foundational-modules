use crate::pb::sol::transactions::v1::Transactions;
use substreams_solana::pb::sf::solana::r#type::v1::{Block, ConfirmedTransaction};

impl Transactions {
    pub fn iter(&self) -> impl Iterator<Item = &ConfirmedTransaction> {
        self.transactions.iter()
    }
}

#[substreams::handlers::map]
fn all_transactions_without_votes(blk: Block) -> Result<Transactions, substreams::errors::Error> {
    Ok(Transactions {
        transactions: blk.transactions,
    })
}

#[substreams::handlers::map]
fn filtered_txs_by_instructions_without_votes(
    query: String,
    mut transactions: Transactions,
) -> Result<Transactions, substreams::errors::Error> {
    let query = substreams::expr_matcher(&query);

    transactions.transactions.retain(|trx| {
        trx.instructions()
            .any(|view| query.matches_keys(&vec![format!("program:{}", view.program_id().to_string())]))
    });

    Ok(transactions)
}
