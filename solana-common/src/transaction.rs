use crate::pb::sf::substreams::solana::v1::Transactions;
use substreams_solana::pb::sf::solana::r#type::v1::Block;

#[substreams::handlers::map]
fn transactions_by_programid_without_votes(
    query: String,
    block: Block,
) -> Result<Transactions, substreams::errors::Error> {
    let query = substreams::expr_matcher(&query);

    let mut transactions = Transactions {
        transactions: block.transactions
    };

    transactions.transactions.retain(|trx| {
        trx.walk_instructions()
            .any(|view| query.matches_keys(&vec![format!("program:{}", view.program_id())]))
    });

    Ok(transactions)
}

#[substreams::handlers::map]
fn transactions_by_programid_and_account_without_votes(
    query: String,
    block: Block,
) -> Result<Transactions, substreams::errors::Error> {
    let query = substreams::expr_matcher(&query);

    let mut transactions = Transactions {
        transactions: block.transactions
    };

    transactions.transactions.retain(|trx| {
        trx.walk_instructions()
            .any(|view| query.matches_keys(
                &view.accounts()
                    .iter()
                    .map(|acc| format!("account:{}",acc))
                    .chain(vec![format!("program:{}", view.program_id())])
                    .collect::<Vec<String>>()
                    
        )
    )});

    Ok(transactions)
}