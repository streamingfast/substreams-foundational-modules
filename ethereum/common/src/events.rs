use anyhow::Ok;
use substreams::errors::Error;
use substreams::pb::sf::substreams::index::v1::Keys;
use substreams::Hex;
use substreams_ethereum::pb::eth::v2::BlockLazyView;

#[substreams::handlers::map]
fn index_events(block: &BlockLazyView<'_>) -> Result<Keys, Error> {
    let mut keys = Keys::default();

    for log in block.logs() {
        keys.keys.extend(evt_keys(&log));
    }

    Ok(keys)
}

/// The log fields the index keys are built from, implemented for both the owned
/// `Log` and buffa's `LogLazyView`.
pub trait EvtKeyed {
    fn first_topic(&self) -> Option<&[u8]>;
    fn key_address(&self) -> &[u8];
}

impl EvtKeyed for substreams_ethereum::pb::eth::v2::Log {
    fn first_topic(&self) -> Option<&[u8]> {
        self.topics.get(0).map(|t| t.as_ref())
    }

    fn key_address(&self) -> &[u8] {
        &self.address
    }
}

impl EvtKeyed for substreams_ethereum::pb::eth::v2::LogLazyView<'_> {
    fn first_topic(&self) -> Option<&[u8]> {
        self.topics.get(0).map(|t| &**t)
    }

    fn key_address(&self) -> &[u8] {
        self.address
    }
}

impl<T: EvtKeyed + ?Sized> EvtKeyed for &T {
    fn first_topic(&self) -> Option<&[u8]> {
        (**self).first_topic()
    }

    fn key_address(&self) -> &[u8] {
        (**self).key_address()
    }
}

pub fn evt_keys<L: EvtKeyed + ?Sized>(log: &L) -> Vec<String> {
    let mut keys = Vec::new();

    if let Some(topic) = log.first_topic() {
        let k_log_sign = format!("evt_sig:0x{}", Hex::encode(topic));
        keys.push(k_log_sign);
    }

    let k_log_address = format!("evt_addr:0x{}", Hex::encode(log.key_address()));
    keys.push(k_log_address);

    keys
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use buffa::view::LazyMessageView;

    #[test]
    fn test_index_events() {
        // Given
        let bytes = testing::read_block_bytes("./src/testdata/ethereum_mainnet_10500500.binpb.base64");
        let block = BlockLazyView::decode_lazy(&bytes).expect("Not able to decode Block");

        // When
        let keys = substreams::testing::map!(index_events(&block))
            .expect("Failed to execute function");

        // Expect
        assert!(keys.keys.len() > 0);
        assert!(keys
            .keys
            .iter()
            .all(|k| k.starts_with("evt_sig:0x") || k.starts_with("evt_addr:0x")));
        assert!(keys
            .keys
            .iter()
            .any(|k| k == "evt_addr:0x5acc84a3e955bdd76467d3348077d003f00ffb97"));
    }
}
