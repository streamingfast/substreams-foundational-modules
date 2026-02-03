use substreams_solana::{base58, pb::sf::solana::r#type::v1::ConfirmedTransaction};

/// transaction_program_and_account_keys returns an iterator of keys extracted from a transaction. It will
/// emit the account keys from the transaction message, the loaded writable addresses, the loaded readonly
/// addresses, and the program ids from the instructions.
pub fn transaction_program_and_account_keys(
    trx: &ConfirmedTransaction,
) -> impl Iterator<Item = String> + '_ {
    let meta = trx.meta.as_ref().unwrap();
    let message = trx.transaction.as_ref().unwrap().message.as_ref().unwrap();

    message
        .account_keys
        .iter()
        .chain(meta.loaded_writable_addresses.iter())
        .chain(meta.loaded_readonly_addresses.iter())
        .map(|acct| {
            let encoded = base58::encode(acct);
            let mut result = String::with_capacity(8 + encoded.len());
            result.push_str("account:");
            result.push_str(&encoded);
            result
        })
        .chain(trx.walk_instructions().map(|inst| {
            let program_id = inst.program_id().to_string();
            let mut result = String::with_capacity(8 + program_id.len());
            result.push_str("program:");
            result.push_str(&program_id);
            result
        }))
}

/// Optimized version that collects keys into a pre-allocated vector
/// to reduce allocations in hot paths
pub(crate) fn transaction_program_and_account_keys_vec(trx: &ConfirmedTransaction) -> Vec<String> {
    let meta = trx.meta.as_ref().unwrap();
    let message = trx.transaction.as_ref().unwrap().message.as_ref().unwrap();

    // Pre-calculate capacity to avoid reallocations
    let account_count = message.account_keys.len()
        + meta.loaded_writable_addresses.len()
        + meta.loaded_readonly_addresses.len();

    // Estimate instruction count (most transactions have 1-10 instructions)
    let estimated_instruction_count = 10;
    let estimated_capacity = account_count + estimated_instruction_count;

    let mut keys = Vec::with_capacity(estimated_capacity);

    // Process all account keys
    for acct in message
        .account_keys
        .iter()
        .chain(meta.loaded_writable_addresses.iter())
        .chain(meta.loaded_readonly_addresses.iter())
    {
        let encoded = base58::encode(acct);
        let mut key = String::with_capacity(8 + encoded.len());
        key.push_str("account:");
        key.push_str(&encoded);
        keys.push(key);
    }

    // Process all program IDs
    for inst in trx.walk_instructions() {
        let program_id = inst.program_id().to_string();
        let mut key = String::with_capacity(8 + program_id.len());
        key.push_str("program:");
        key.push_str(&program_id);
        keys.push(key);
    }

    keys
}

#[cfg(test)]
mod tests {
    use super::*;
    use substreams_solana::pb::sf::solana::r#type::v1::Block;

    #[test]
    fn test_transaction_program_and_account_keys() {
        // Given
        let block: Block =
            testing::read_block("./src/testdata/solana_mainnet_313000000.binpb.base64");
        let confirmed_transaction = block.transactions.get(8).unwrap();

        // When
        let mut result = transaction_program_and_account_keys(confirmed_transaction);

        // Expected
        if let Some(tx) = confirmed_transaction.transaction.as_ref() {
            if let Some(msg) = tx.message.as_ref() {
                msg.account_keys.iter().for_each(|acct| {
                    assert_eq!(
                        result.any(|index| index == format!("account:{}", base58::encode(acct))),
                        true
                    )
                });
            }
        }

        confirmed_transaction.walk_instructions().for_each(|inst| {
            let expected_key = format!("program:{}", inst.program_id());
            assert_eq!(result.any(|index| index == expected_key), true)
        });
    }
}
