use crate::pb::sol::transactions::v1::Transactions;
use substreams_solana::pb::sf::solana::r#type::v1::Block;

#[substreams::handlers::map]
fn filtered_transactions_without_votes(
    query: String,
    block: Block,
) -> Result<Transactions, substreams::errors::Error> {
    let query = substreams::expr_matcher(&query);

    let mut transactions = Transactions {
        transactions: block.transactions
    };

    transactions.transactions.retain(|trx| {
        trx.walk_instructions()
            .any(|view| query.matches_keys(&vec![format!("program:{}", view.program_id().to_string())]))
    });

    Ok(transactions)
}