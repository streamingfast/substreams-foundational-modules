use substreams_solana::b58;
use substreams_solana::pb::sf::solana::r#type::v1::Block;

static VOTE_INSTRUCTION: [u8; 32] = b58!("Vote111111111111111111111111111111111111111");

#[substreams::handlers::map]
fn blocks_without_votes(mut block: Block) -> Result<Block, substreams::errors::Error> {
    block.transactions.retain(|trx| {
        if trx.meta.is_unset() || trx.transaction.is_unset() || trx.transaction.message.is_unset() {
            return false;
        }
        if trx.meta.err.is_set() {
            return false;
        }

        let message = &trx.transaction.message;

        // Retain only transactions that do **not** contain a vote instruction
        !message.account_keys.iter().any(|v| v == &VOTE_INSTRUCTION)
    });

    Ok(block)
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_block_without_votes() {
        // Given
        let block = testing::read_block("./src/testdata/solana_mainnet_313000000.binpb.base64");

        // When
        let result = substreams::testing::map!(blocks_without_votes(block))
            .expect("Failed to execute function");

        // Expect
        result.transactions().for_each(|t| {
            assert_eq!(
                t.transaction
                    .clone()
                    .unwrap()
                    .message
                    .unwrap()
                    .account_keys
                    .iter()
                    .all(|acct| acct != &VOTE_INSTRUCTION),
                true
            )
        });
    }
}
