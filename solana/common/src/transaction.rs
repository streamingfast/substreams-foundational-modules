use crate::{
    keys::lazy_transaction_program_and_account_keys_vec,
    pb::sf::substreams::solana::v1::Transactions,
};
use buffa::view::LazyMessageView;
use substreams_solana::block_view::lazy::LazyTransaction;
use substreams_solana::pb::sf::solana::r#type::v1::BlockLazyView;

#[substreams::handlers::map]
fn transactions_by_programid_without_votes(
    query: String,
    block: &BlockLazyView<'_>,
) -> Result<Transactions, substreams::errors::Error> {
    let query = substreams::sqe::expr_matcher(&query);

    let mut transactions = Vec::new();
    for trx in block.transactions() {
        let resolved = LazyTransaction::new(&trx)?;
        let matched = resolved.walk_instructions()?.any(|view| {
            let key = format!("program:{}", view.program_id());
            query.matches_keys(&[key])
        });

        if matched {
            transactions.push(trx.to_owned_message()?);
        }
    }

    Ok(Transactions { transactions })
}

#[substreams::handlers::map]
fn transactions_by_programid_and_account_without_votes(
    query: String,
    block: &BlockLazyView<'_>,
) -> Result<Transactions, substreams::errors::Error> {
    let query = substreams::sqe::expr_matcher(&query);

    let mut transactions = Vec::new();
    for trx in block.transactions() {
        let resolved = LazyTransaction::new(&trx)?;
        let keys = lazy_transaction_program_and_account_keys_vec(&resolved);

        if query.matches_keys(&keys) {
            transactions.push(trx.to_owned_message()?);
        }
    }

    Ok(Transactions { transactions })
}

#[cfg(test)]
mod tests {
    use substreams_solana::{base58, pb::sf::solana::r#type::v1::ConfirmedTransaction};

    use super::*;

    #[test]
    fn test_transactions_by_programid_without_votes() {
        // Given
        let bytes =
            testing::read_block_bytes("./src/testdata/solana_mainnet_313000000.binpb.base64");
        let block = BlockLazyView::decode_lazy(&bytes).expect("valid block");

        // When
        let result = substreams::testing::map!(transactions_by_programid_without_votes(
            "program:whirLbMiicVdio4qvUfM5KAg6Ct8VwpYzGff3uctyCc".to_owned(),
            &block,
        ))
        .expect("Failed to execute function");

        // Expect
        const EXPECTED_PROGRAM_ID: &str = "whirLbMiicVdio4qvUfM5KAg6Ct8VwpYzGff3uctyCc";
        result.transactions.into_iter().for_each(|transaction| {
            assert_eq!(
                transaction
                    .walk_instructions()
                    .any(|instruction| instruction.program_id().to_string() == EXPECTED_PROGRAM_ID),
                true
            )
        });
    }

    #[test]
    fn test_transactions_by_programid_and_account_without_votes() {
        // Given
        let bytes =
            testing::read_block_bytes("./src/testdata/solana_mainnet_313000000.binpb.base64");
        let block = BlockLazyView::decode_lazy(&bytes).expect("valid block");

        // When
        let result = substreams::testing::map!(transactions_by_programid_and_account_without_votes(
            "program:whirLbMiicVdio4qvUfM5KAg6Ct8VwpYzGff3uctyCc && account:5qrvgpvr55Eo7c5bBcwopdiQ6TpvceiRm42yjHTbtDvc".to_owned(),
            &block,
        ))
        .expect("Failed to execute function");

        // Expect
        result.transactions.into_iter().for_each(|transaction| {
            let mut matched = true;

            if let Some(tx) = transaction.transaction.as_option() {
                if let Some(msg) = tx.message.as_option() {
                    if !msg.account_keys.iter().any(|acct| {
                        base58::encode(acct) == "5qrvgpvr55Eo7c5bBcwopdiQ6TpvceiRm42yjHTbtDvc"
                    }) {
                        matched = false
                    }
                }
            }

            // Check if the given program id is contained within the instructions.
            const EXPECTED_PROGRAM_ID: &str = "whirLbMiicVdio4qvUfM5KAg6Ct8VwpYzGff3uctyCc";
            const EXPECTED_ACCOUNT: &str = "5qrvgpvr55Eo7c5bBcwopdiQ6TpvceiRm42yjHTbtDvc";

            if !transaction
                .walk_instructions()
                .any(|instruction| instruction.program_id().to_string() == EXPECTED_PROGRAM_ID)
            {
                matched = false
            }

            // For all the instructions of the given program id, check if the account is contained
            transaction
                .walk_instructions()
                .filter(|instruction| instruction.program_id().to_string() == EXPECTED_PROGRAM_ID)
                .for_each(|instruction| {
                    matched = instruction
                        .accounts()
                        .iter()
                        .any(|account| account.to_string() == EXPECTED_ACCOUNT)
                });

            assert_eq!(matched, true)
        });
    }

    #[test]
    fn test_transactions_by_programid_and_account_without_votes_account_keys() {
        // Given
        let bytes =
            testing::read_block_bytes("./src/testdata/solana_mainnet_318251413.binpb.base64");
        let view = BlockLazyView::decode_lazy(&bytes).expect("valid block");
        let block: substreams_solana::pb::sf::solana::r#type::v1::Block =
            testing::read_block("./src/testdata/solana_mainnet_318251413.binpb.base64");

        // When
        let result = substreams::testing::map!(transactions_by_programid_and_account_without_votes(
            "program:JUP6LkbZbjS1jKKwapdHNy74zcZ3tLUZoi5QNyVTaV4 && account:3EsvvyqporKr5DVpzWsdYCphpXqXnQBMQLGNwSH5MmRE".to_owned(),
            &view,
        ))
        .expect("Failed to execute function");

        // Expect
        const EXPECTED_PROGRAM_ID_JUP: &str = "JUP6LkbZbjS1jKKwapdHNy74zcZ3tLUZoi5QNyVTaV4";
        const EXPECTED_ACCOUNT_JUP: &str = "3EsvvyqporKr5DVpzWsdYCphpXqXnQBMQLGNwSH5MmRE";

        let expected_transactions: Vec<&ConfirmedTransaction> = block
            .transactions()
            .filter(|transaction| {
                let mut matched = true;

                if !transaction
                    .walk_instructions()
                    .any(|inst| inst.program_id().to_string() == EXPECTED_PROGRAM_ID_JUP)
                {
                    matched = false;
                }

                if let Some(tx) = transaction.transaction.as_option() {
                    if let Some(msg) = tx.message.as_option() {
                        if !msg
                            .account_keys
                            .iter()
                            .any(|acct| base58::encode(acct) == EXPECTED_ACCOUNT_JUP)
                        {
                            matched = false;
                        }
                    }
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
