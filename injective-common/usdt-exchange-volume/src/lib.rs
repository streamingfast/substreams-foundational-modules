mod pb;

use crate::pb::sf::cosmos::r#type::v2::Block;
use anyhow::anyhow;
use cosmrs::Any;
use cosmrs::Tx;
use pb::sf::cosmos::r#type::v2::Event;
use pb::sf::cosmos::r#type::v2::TxResults;
use pb::sf::substreams::cosmos::v1::UsdtExchange;
use pb::sf::substreams::cosmos::v1::UsdtExchangeList;
use substreams::errors::Error;

const DOJO_SMART_CONTRACT: &str = "inj1h0mpv48ctcsmydymh2hnkal7hla5gl4gftemqv";
const USDT_ADDRESS: &str = "peggy0xdAC17F958D2ee523a2206206994597C13D831ec7";

#[substreams::handlers::map]
pub fn map_usdt_exchanges(block: Block) -> Result<UsdtExchangeList, Error> {
    // Mutable list to add the output of the Substreams
    let mut usdt_exchanges: Vec<UsdtExchange> = Vec::new();
    substreams::log::println("1");
    if block.txs.len() != block.tx_results.len() {
        return Err(anyhow!("Transaction list and result list do not match"));
    }

    for i in 0..block.txs.len() {
        let tx = block.txs.get(i).unwrap();
        let tx_result = block.tx_results.get(i).unwrap();

        if let Ok(transaction) = Tx::from_bytes(tx) {
            for message in transaction.body.messages {
                if let Some(usdt_exchange) = handle_msg_execute_contract(&message, &tx_result) {
                    substreams::log::println(usdt_exchange.amount.to_string());
                    usdt_exchanges.push(usdt_exchange)
                }
            }
        }
    }
   
    Ok(UsdtExchangeList {
        exchanges: usdt_exchanges,
    })
}

fn handle_msg_execute_contract(message: &Any, tx_result: &TxResults) -> Option<UsdtExchange> {
    if message.type_url != "/cosmwasm.wasm.v1.MsgExecuteContract"
        && message.type_url != "/injective.wasmx.v1.MsgExecuteContractCompat"
    {
        return None;
    }

    if !is_dojo_smart_contract(&tx_result) {
        return None;
    }

    if let Some(event) = tx_result.events.iter().find(|event| event.r#type == "wasm") {
        return extract_data_from_event(event);
    }

    return None;
}

fn is_dojo_smart_contract(tx_result: &TxResults) -> bool {
    if let Some(event) = tx_result.events.iter().find(|event| event.r#type == "execute") {
        if let Some(attr) = event.attributes.iter().find(|attr| attr.key == "_contract_address") {
            return attr.value == DOJO_SMART_CONTRACT;
        }
    }

    return false;
}

fn extract_data_from_event(event: &Event) -> Option<UsdtExchange> {
    let mut offer_asset = &String::new();
    let mut offer_amount = 0;
    let mut ask_asset = &String::new();
    let mut ask_amount = 0;

    event.attributes.iter().for_each(|att| {
        if att.key == "offer_asset" {
            offer_asset = &att.value;
        }

        if att.key == "offer_amount" {
            offer_amount = att.value.parse().unwrap();
        }

        if att.key == "ask_asset" {
            ask_asset = &att.value;
        }

        if att.key == "ask_amount" {
            ask_amount = att.value.parse().unwrap();
        }
    });

    if ask_asset == USDT_ADDRESS {
        substreams::log::println(ask_amount.to_string());
        return Some(UsdtExchange { amount: ask_amount });
    }

    if offer_asset == USDT_ADDRESS {
        substreams::log::println(offer_amount.to_string());
        return Some(UsdtExchange { amount: offer_amount });
    }

    return None;
}
