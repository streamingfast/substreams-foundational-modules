#[allow(dead_code, unused_imports)]
mod pb;
mod rpc;

use buffa_types::google::protobuf::Any;
use pb::evm::erc20::metadata::v1 as original;
use pb::sf::substreams::ethereum::erc20::v1::TokenMetadata;
use substreams::pb::sf::substreams::foundational_store::model::v2::{Entry, Key, SinkEntries};

#[substreams::handlers::map]
fn metadata_to_foundational_store(
    events: original::Events,
) -> Result<SinkEntries, substreams::errors::Error> {
    let mut entries = Vec::new();

    // For MetadataInitialize events, we can create TokenMetadata directly from the event
    for init in events.metadata_initialize {
        let token_metadata = TokenMetadata {
            address: init.address.clone(),
            name: init.name.unwrap_or_default(),
            symbol: init.symbol.unwrap_or_default(),
            decimals: init.decimals,
        };

        let any = Any::pack(
            &token_metadata,
            "type.googleapis.com/sf.substreams.ethereum.erc20.v1.TokenMetadata",
        );

        let entry = Entry {
            key: Key {
                bytes: init.address,
            }
            .into(),
            value: any.into(),
        };

        entries.push(entry);
    }

    // For MetadataChanges events, we need to fetch the full metadata via RPC
    let mut change_addresses = Vec::new();
    for change in events.metadata_changes {
        change_addresses.push(change.address);
    }

    if !change_addresses.is_empty() {
        let address_refs: Vec<&[u8]> = change_addresses
            .iter()
            .map(|addr| addr.as_slice())
            .collect();

        // Batch RPC calls to get metadata
        let names = rpc::batch_name(&address_refs, 50);
        let symbols = rpc::batch_symbol(&address_refs, 50);
        let decimals = rpc::batch_decimals(&address_refs, 50);

        // Create entries for addresses that had changes
        for address_bytes in &change_addresses {
            let address_slice = address_bytes.as_slice();
            let name = names.get(address_slice).cloned().unwrap_or_default();
            let symbol = symbols.get(address_slice).cloned().unwrap_or_default();
            let decimals_val = decimals.get(address_slice).cloned().unwrap_or(0);

            let token_metadata = TokenMetadata {
                address: address_bytes.clone(),
                name,
                symbol,
                decimals: decimals_val,
            };

            let any = Any::pack(
                &token_metadata,
                "type.googleapis.com/evm.token.metadata.v1.TokenMetadata",
            );

            let entry = Entry {
                key: Key {
                    bytes: address_bytes.clone(),
                }
                .into(),

                value: any.into(),
            };

            entries.push(entry);
        }
    }

    Ok(SinkEntries {
        entries,
        if_not_exist: false,
    })
}
