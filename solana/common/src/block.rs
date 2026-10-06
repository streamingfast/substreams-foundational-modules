use buffa::view::LazyMessageView;
use substreams_solana::b58;
use substreams_solana::pb::sf::solana::r#type::v1::{Block, BlockLazyView};

static VOTE_INSTRUCTION: [u8; 32] = b58!("Vote111111111111111111111111111111111111111");

#[substreams::handlers::map]
fn blocks_without_votes(block: &BlockLazyView<'_>) -> Result<Block, substreams::errors::Error> {
    let mut out = block.to_owned_message()?;

    out.transactions.retain(|trx| {
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

    Ok(out)
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_block_without_votes() {
        // Given
        let bytes =
            testing::read_block_bytes("./src/testdata/solana_mainnet_313000000.binpb.base64");
        let block = BlockLazyView::decode_lazy(&bytes).expect("valid block");

        // When
        let result = substreams::testing::map!(blocks_without_votes(&block))
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
