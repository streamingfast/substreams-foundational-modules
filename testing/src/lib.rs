use std::fs;

use base64::{prelude::BASE64_STANDARD, Engine};
use buffa::Message;

pub fn read_block<B: Message + Default>(filename: &str) -> B {
    let encoded = fs::read_to_string(filename).expect("Failed to read file");
    let raw_bytes = BASE64_STANDARD
        .decode(&encoded)
        .expect("Failed to decode base64");

    B::decode_from_slice(&raw_bytes).expect("Not able to decode Block")
}
