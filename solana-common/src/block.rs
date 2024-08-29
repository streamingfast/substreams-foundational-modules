use substreams_solana::b58;
use substreams_solana::pb::sf::solana::r#type::v1::Block;

static VOTE_INSTRUCTION: [u8; 32] = b58!("Vote111111111111111111111111111111111111111");

#[substreams::handlers::map]
fn blocks_without_votes(mut block: Block) -> Result<Block, substreams::errors::Error> {
    block.transactions.retain(|trx| {
        let meta = match trx.meta.as_ref() {
            Some(meta) => meta,
            None => return false,
        };
        if meta.err.is_some() {
            return false;
        }

        let transaction = match trx.transaction.as_ref() {
            Some(transaction) => transaction,
            None => return false,
        };
        let message = transaction.message.as_ref().expect("Message is missing");

        // Retain only transactions that do **not** contain a vote instruction
        !message.account_keys.iter().any(|v| v == &VOTE_INSTRUCTION)
    });

    Ok(block)
}
