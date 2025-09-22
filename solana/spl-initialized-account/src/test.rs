#[cfg(test)]
mod tests {
    use crate::pb::sol::transactions::v1::Transactions as SolanaTransactions;
    use crate::{_map_spl_initialized_account, SOLANA_TOKEN_PROGRAM_KEG, SOLANA_TOKEN_PROGRAM_ZQB};
    use prost::Message;
    use substreams_solana::pb::sf::solana::r#type::v1::Block;

    #[test]
    fn test_map_spl_initialized_account() {
        let block: Block =
            testing::read_block("./src/testdata/solana_mainnet_313000000.binpb.base64");

        let filtered_transactions: Vec<_> = block
            .transactions
            .into_iter()
            .filter(|trx| {
                if let Some(meta) = &trx.meta {
                    if meta.err.is_some() {
                        return false;
                    }
                }

                if let Some(transaction) = &trx.transaction {
                    if let Some(message) = &transaction.message {
                        return message.account_keys.iter().any(|key| {
                            let key_str = bs58::encode(key).into_string();
                            key_str == SOLANA_TOKEN_PROGRAM_KEG
                                || key_str == SOLANA_TOKEN_PROGRAM_ZQB
                        });
                    }
                }
                false
            })
            .filter_map(|trx| {
                let mut buf = Vec::new();
                if Message::encode(&trx, &mut buf).is_ok() {
                    if let Ok(converted) =
                        crate::pb::sf::solana::r#type::v1::ConfirmedTransaction::decode(&buf[..])
                    {
                        return Some(converted);
                    }
                }
                None
            })
            .collect();

        let transactions = SolanaTransactions {
            transactions: filtered_transactions,
        };

        let result =
            _map_spl_initialized_account(transactions).expect("Failed to execute function");

        for entry in result.entries.iter() {
            assert!(!entry.key.is_empty(), "Entry key should not be empty");
            assert!(entry.value.is_some(), "Entry value should not be None");

            if let Some(value) = &entry.value {
                assert_eq!(
                    value.type_url, "type.googleapis.com/sf.substreams.solana.spl.v1.AccountOwner",
                    "Type URL should match AccountOwner"
                );
                assert!(!value.value.is_empty(), "Value should not be empty");
            }
        }
    }
}
