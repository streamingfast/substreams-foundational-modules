mod pb;

use prost::Message;
use std::collections::HashSet;
use std::fmt::Error;
use substreams::store::FoundationalStore;
use substreams::{log, Hex};
use substreams_ethereum::pb::eth::v2::{Block, TransactionTraceStatus};

use crate::pb::erc20::metadata::v1::{TokenMeta, TokenTransfer, TokenTransfers};
use crate::pb::evm::token::metadata::v1::TokenMetadata;
use substreams::pb::sf::substreams::foundational_store::v1::ResponseCode;

substreams_ethereum::init!();

// ERC20 Transfer event signature hash
const ERC20_TRANSFER_EVENT_HASH: [u8; 32] = [
    0xdd, 0xf2, 0x52, 0xad, 0x1b, 0xe2, 0xc8, 0x9b, 0x69, 0xc2, 0xb0, 0x68, 0xfc, 0x37, 0x8d, 0xaa,
    0x95, 0x2b, 0xa7, 0xf1, 0x63, 0xc4, 0xa1, 0x16, 0x28, 0xf5, 0x5a, 0x4d, 0xf5, 0x23, 0xb3, 0xef,
];

#[substreams::handlers::map]
fn map_tokens_transfers(
    blk: Block,
    foundational_store: FoundationalStore,
) -> Result<TokenTransfers, substreams::errors::Error> {
    let mut transfers = Vec::new();
    let mut token_addresses_to_resolve = HashSet::new();

    for trx in blk.transaction_traces.iter() {
        let status = TransactionTraceStatus::try_from(trx.status)?;
        if status != TransactionTraceStatus::Succeeded {
            continue
        }

        if let Some(receipt) = &trx.receipt {
            for (log_index, log) in receipt.logs.iter().enumerate() {
                // Check if this is an ERC20 Transfer event
                if log.topics.len() >= 3 && log.topics[0] == ERC20_TRANSFER_EVENT_HASH.to_vec() {
                    token_addresses_to_resolve.insert(log.address.clone());

                    // Decode Transfer event: Transfer(address indexed from, address indexed to, uint256 value)
                    // topics[0] = event signature
                    // topics[1] = from address (32 bytes, last 20 are the address)
                    // topics[2] = to address (32 bytes, last 20 are the address)
                    // data = amount (32 bytes, uint256)

                    let from = if log.topics[1].len() >= 20 {
                        log.topics[1][12..].to_vec()
                    } else {
                        continue;
                    };

                    let to = if log.topics[2].len() >= 20 {
                        log.topics[2][12..].to_vec()
                    } else {
                        continue;
                    };

                    let amount = if log.data.len() >= 32 {
                        let amount_bytes = &log.data[0..32];
                        format!("0x{}", Hex::encode(amount_bytes))
                    } else {
                        "0x0".to_string()
                    };

                    transfers.push((
                        log.address.clone(),
                        from,
                        to,
                        amount,
                        trx.hash.clone(),
                        log_index as u32,
                    ));
                }
            }
        }
    }

    let mut transfers_with_metadata = Vec::new();

    if !token_addresses_to_resolve.is_empty() {
        // Batch lookup metadata from foundational store
        let keys_to_query: Vec<Vec<u8>> = token_addresses_to_resolve.into_iter().collect();
        let resp = foundational_store.get_all(&keys_to_query);

        // Create a map of address -> metadata for quick lookup
        let mut metadata_map = std::collections::HashMap::new();
        for entry in resp.entries {
            let code = ResponseCode::try_from(entry.response.as_ref().unwrap().response)?;
            if code != ResponseCode::Found {
                return Err(anyhow::Error::msg(format!(
                    "Token meta not found for address: 0x{:?}",
                    Hex(entry.key)
                )));
            }

            if let Ok(token_metadata) = TokenMetadata::decode(entry.response.unwrap().value.unwrap().value.as_slice())
            {
                metadata_map.insert(entry.key, token_metadata);
            }
        }

        // Enrich transfers with metadata
        for (token_address, from, to, amount, tx_hash, log_index) in transfers {
            let metadata = match metadata_map.get(&token_address) {
                Some(m) => Some(TokenMeta {
                    name: m.name.clone(),
                    symbol: m.symbol.clone(),
                    decimals: m.decimals,
                }),
                None => None,
            };
            let enriched_transfer = TokenTransfer {
                token_address: token_address.clone(),
                metadata,
                from,
                to,
                amount,
                block_number: blk.number,
                tx_hash: format!("0x{}", Hex::encode(&tx_hash)),
                log_index,
            };

            transfers_with_metadata.push(enriched_transfer);
        }
    }

    Ok(TokenTransfers {
        transfers: transfers_with_metadata,
    })
}
