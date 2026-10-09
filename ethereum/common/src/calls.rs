use crate::pb::sf::substreams::ethereum::v1::{Call, Calls};
use crate::pb::sf::substreams::v1::Clock;
use anyhow::Ok;
use buffa::view::{LazyMessageView, MessageView};
use substreams::errors::Error;
use substreams::pb::sf::substreams::index::v1::Keys;
use substreams::Hex;
use substreams_ethereum::pb::eth::v2::BlockLazyView;

#[substreams::handlers::map]
fn index_calls(block: &BlockLazyView<'_>) -> Result<Keys, Error> {
    let mut keys = Keys::default();

    for call in block.calls() {
        keys.keys.extend(call_keys(&call));
    }

    Ok(keys)
}

#[substreams::handlers::map]
fn filtered_calls(query: String, block: &BlockLazyView<'_>) -> Result<Calls, Error> {
    let matcher = substreams::sqe::expr_matcher(&query);

    let mut calls = Vec::new();
    for trace in block.transactions() {
        for call in trace.calls.iter() {
            let call = call?;
            let keys = call_keys(&call);
            let keys = keys.iter().map(|k| k.as_str()).collect::<Vec<&str>>();
            if !matcher.matches_keys(&keys) {
                continue;
            }

            calls.push(Call {
                tx_hash: Hex::encode(&trace.hash),
                call: call.to_owned_message()?.into(),
            });
        }
    }

    let timestamp = match block.header.get()? {
        Some(header) => header.timestamp.to_owned_message()?.into(),
        None => Default::default(),
    };

    Ok(Calls {
        calls,
        clock: Clock {
            timestamp,
            id: Hex::encode(&block.hash),
            number: block.number,
        }
        .into(),
    })
}


/// The call fields the index keys are built from, implemented for both the owned
/// `Call` and buffa's `CallLazyView`.
pub trait CallKeyed {
    fn key_caller(&self) -> &[u8];
    fn key_address(&self) -> &[u8];
    fn key_input(&self) -> &[u8];
}

impl CallKeyed for substreams_ethereum::pb::eth::v2::Call {
    fn key_caller(&self) -> &[u8] {
        &self.caller
    }

    fn key_address(&self) -> &[u8] {
        &self.address
    }

    fn key_input(&self) -> &[u8] {
        &self.input
    }
}

impl CallKeyed for substreams_ethereum::pb::eth::v2::CallLazyView<'_> {
    fn key_caller(&self) -> &[u8] {
        self.caller
    }

    fn key_address(&self) -> &[u8] {
        self.address
    }

    fn key_input(&self) -> &[u8] {
        self.input
    }
}

pub fn call_keys<C: CallKeyed + ?Sized>(call: &C) -> Vec<String> {
    let mut keys = Vec::new();

    let k_call_from = format!("call_from:0x{}", Hex::encode(call.key_caller()));
    keys.push(k_call_from);

    let k_call_to = format!("call_to:0x{}", Hex::encode(call.key_address()));
    keys.push(k_call_to);

    let input_bytes = call.key_input();

    if input_bytes.len() >= 4 {
        let k_call_method = format!("call_method:0x{}", Hex::encode(&input_bytes[..4]));
        keys.push(k_call_method);
    }

    keys
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use buffa::view::LazyMessageView;
    use substreams_ethereum::pb::eth::v2::BlockLazyView;

    /// The fixture block holds 670 calls across its successful transactions, so
    /// `call_keys` yields one `call_from` and one `call_to` for each.
    #[test]
    fn test_call_keys_over_a_real_block() {
        let bytes =
            testing::read_block_bytes("./src/testdata/ethereum_mainnet_10500500.binpb.base64");
        let block = BlockLazyView::decode_lazy(&bytes).expect("Not able to decode Block");

        let keys: Vec<String> = block.calls().flat_map(|call| call_keys(&call)).collect();

        assert_eq!(keys.iter().filter(|k| k.starts_with("call_from:")).count(), 670);
        assert_eq!(keys.iter().filter(|k| k.starts_with("call_to:")).count(), 670);
        assert!(keys.iter().any(|k| k == "call_from:0x5acc84a3e955bdd76467d3348077d003f00ffb97"));
    }

    /// `index_calls` is `call_keys` over every call of the block, so it carries the
    /// two keys per call that the test above counts.
    #[test]
    fn test_index_calls() {
        let bytes =
            testing::read_block_bytes("./src/testdata/ethereum_mainnet_10500500.binpb.base64");
        let block = BlockLazyView::decode_lazy(&bytes).expect("Not able to decode Block");

        let keys = substreams::testing::map!(index_calls(&block)).expect("Failed to execute");

        assert_eq!(
            keys.keys.iter().filter(|k| k.starts_with("call_from:")).count(),
            670
        );
        assert_eq!(
            keys.keys.iter().filter(|k| k.starts_with("call_to:")).count(),
            670
        );
        assert!(keys.keys.iter().all(|k| {
            k.starts_with("call_from:0x")
                || k.starts_with("call_to:0x")
                || k.starts_with("call_method:0x")
        }));
    }

    /// A query that names one caller keeps only that caller's calls, and a query that
    /// can match nothing keeps none.
    #[test]
    fn test_filtered_calls() {
        let bytes =
            testing::read_block_bytes("./src/testdata/ethereum_mainnet_10500500.binpb.base64");
        let block = BlockLazyView::decode_lazy(&bytes).expect("Not able to decode Block");

        let query = "call_from:0x5acc84a3e955bdd76467d3348077d003f00ffb97".to_string();
        let calls =
            substreams::testing::map!(filtered_calls(query, &block)).expect("Failed to execute");

        assert!(!calls.calls.is_empty());
        assert!(calls.calls.iter().all(|c| {
            Hex::encode(&c.call.caller) == "5acc84a3e955bdd76467d3348077d003f00ffb97"
        }));
        assert_eq!(calls.clock.number, 10500500);

        let none = substreams::testing::map!(filtered_calls(
            "call_from:0x0000000000000000000000000000000000000000".to_string(),
            &block
        ))
        .expect("Failed to execute");
        assert!(none.calls.is_empty());
    }
}
