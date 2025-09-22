use crate::{
    keys::transaction_program_and_account_keys, pb::sf::substreams::solana::v1::Transactions,
};
use substreams_solana::pb::sf::solana::r#type::v1::Block;

#[substreams::handlers::map]
fn transactions_by_programid_without_votes(
    query: String,
    block: Block,
) -> Result<Transactions, substreams::errors::Error> {
    _transactions_by_programid_without_votes(query, block)
}

/// _transactions_by_programid_without_votes is equal to [transactions_by_programid_without_votes] but exists only for unit testing purposes.
fn _transactions_by_programid_without_votes(
    query: String,
    block: Block,
) -> Result<Transactions, substreams::errors::Error> {
    let query = substreams::expr_matcher(&query);

    let mut transactions = Transactions {
        transactions: block.transactions,
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
    _transactions_by_programid_and_account_without_votes(query, block)
}

/// transactions_by_programid_and_account_without_votes is equal to [transactions_by_programid_and_account_without_votes] but exists only for unit testing purposes.
fn _transactions_by_programid_and_account_without_votes(
    query: String,
    block: Block,
) -> Result<Transactions, substreams::errors::Error> {
    let query = substreams::expr_matcher(&query);

    let mut transactions = Transactions {
        transactions: block.transactions,
    };

    transactions.transactions.retain(|trx| {
        let keys: Vec<_> = transaction_program_and_account_keys(trx).collect();

        query.matches_keys(&keys)
    });

    Ok(transactions)
}

#[cfg(test)]
mod tests {
    use substreams_solana::{base58, pb::sf::solana::r#type::v1::ConfirmedTransaction};

    use super::*;

    #[test]
    fn test_transactions_by_programid_without_votes() {
        // Given
        let block = testing::read_block("./src/testdata/solana_mainnet_313000000.binpb.base64");

        // When
        let result = _transactions_by_programid_without_votes(
            "program:whirLbMiicVdio4qvUfM5KAg6Ct8VwpYzGff3uctyCc".to_owned(),
            block,
        )
        .expect("Failed to execute function");

        // Expect
        result.transactions.into_iter().for_each(|transaction| {
            assert_eq!(
                transaction
                    .walk_instructions()
                    .any(|instruction| instruction.program_id().to_string()
                        == "whirLbMiicVdio4qvUfM5KAg6Ct8VwpYzGff3uctyCc"),
                true
            )
        });
    }

    #[test]
    fn test_transactions_by_programid_and_account_without_votes() {
        // Given
        let block = testing::read_block("./src/testdata/solana_mainnet_313000000.binpb.base64");

        // When
        let result = _transactions_by_programid_and_account_without_votes(
            "program:whirLbMiicVdio4qvUfM5KAg6Ct8VwpYzGff3uctyCc && account:5qrvgpvr55Eo7c5bBcwopdiQ6TpvceiRm42yjHTbtDvc".to_owned(),
            block,
        )
        .expect("Failed to execute function");

        // Expect
        result.transactions.into_iter().for_each(|transaction| {
            let mut matched = true;

            if !transaction
                .transaction
                .clone()
                .unwrap()
                .message
                .unwrap()
                .account_keys
                .iter()
                .any(|acct| base58::encode(acct) == "5qrvgpvr55Eo7c5bBcwopdiQ6TpvceiRm42yjHTbtDvc")
            {
                matched = false
            }

            // Check if the given program id is contained within the instructions.
            if !transaction.walk_instructions().any(|instruction| {
                instruction.program_id().to_string()
                    == "whirLbMiicVdio4qvUfM5KAg6Ct8VwpYzGff3uctyCc"
            }) {
                matched = false
            }

            // For all the instructions of the given program id, check if the account is contained
            transaction
                .walk_instructions()
                .filter(|instruction| {
                    instruction.program_id().to_string()
                        == "whirLbMiicVdio4qvUfM5KAg6Ct8VwpYzGff3uctyCc"
                })
                .for_each(|instruction| {
                    matched = instruction.accounts().iter().any(|account| {
                        account.to_string() == "5qrvgpvr55Eo7c5bBcwopdiQ6TpvceiRm42yjHTbtDvc"
                    })
                });

            assert_eq!(matched, true)
        });
    }

    #[test]
    fn test_transactions_by_programid_and_account_without_votes_account_keys() {
        // Given
        let block: Block =
            testing::read_block("./src/testdata/solana_mainnet_318251413.binpb.base64");

        // When
        let result = _transactions_by_programid_and_account_without_votes(
            "program:JUP6LkbZbjS1jKKwapdHNy74zcZ3tLUZoi5QNyVTaV4 && account:3EsvvyqporKr5DVpzWsdYCphpXqXnQBMQLGNwSH5MmRE".to_owned(),
            block.clone(),
        )
        .expect("Failed to execute function");

        // Expect

        let expected_transactions: Vec<&ConfirmedTransaction> = block
            .transactions()
            .filter(|transaction| {
                let mut matched = true;

                if !transaction.walk_instructions().any(|inst| {
                    inst.program_id().to_string() == "JUP6LkbZbjS1jKKwapdHNy74zcZ3tLUZoi5QNyVTaV4"
                }) {
                    matched = false;
                }

                if !transaction
                    .transaction
                    .clone()
                    .unwrap()
                    .message
                    .unwrap()
                    .account_keys
                    .iter()
                    .any(|acct| {
                        base58::encode(acct) == "3EsvvyqporKr5DVpzWsdYCphpXqXnQBMQLGNwSH5MmRE"
                    })
                {
                    matched = false;
                }

                return matched;
            })
            .collect();

        assert!(result.transactions.len() > 0);
        assert_eq!(result.transactions.len(), expected_transactions.len());
        result.transactions.iter().for_each(|result_transaction| {
            assert_eq!(
                expected_transactions.iter().any(
                    |expected_transaction| expected_transaction.id() == result_transaction.id()
                ),
                true
            )
        });
    }
}
