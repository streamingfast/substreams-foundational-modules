use std::fs;

use base64::{prelude::BASE64_STANDARD, Engine};
use buffa::Message;

pub fn read_block<B: Message + Default>(filename: &str) -> B {
    B::decode_from_slice(&read_block_bytes(filename)).expect("Not able to decode Block")
}

/// The raw block bytes, for tests that decode a lazy view rather than an owned message.
/// A lazy view borrows its input, so the caller must keep these bytes alive.
pub fn read_block_bytes(filename: &str) -> Vec<u8> {
    let encoded = fs::read_to_string(filename).expect("Failed to read file");
    BASE64_STANDARD
        .decode(&encoded)
        .expect("Failed to decode base64")
}
