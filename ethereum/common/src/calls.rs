use substreams::Hex;

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
}
