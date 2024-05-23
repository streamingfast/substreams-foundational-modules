mod pb;

use crate::pb::spkg::cosmos::v1::transaction::message::Value;
use crate::pb::spkg::cosmos::v1::transaction::Message;
use crate::pb::spkg::cosmos::v1::TransactionList;
use pb::sf::cosmos::r#type::v2::Event;
use pb::sf::substreams::cosmos::v1::UsdtExchange;
use pb::sf::substreams::cosmos::v1::UsdtExchangeList;
use substreams::errors::Error;

const DOJO_SMART_CONTRACT: &str = "inj1h0mpv48ctcsmydymh2hnkal7hla5gl4gftemqv";
const USDT_ADDRESS: &str = "peggy0xdAC17F958D2ee523a2206206994597C13D831ec7";

#[substreams::handlers::map]
pub fn map_usdt_exchanges(transaction_list: TransactionList) -> Result<UsdtExchangeList, Error> {
    // Mutable list to add the output of the Substreams
    let mut usdt_exchanges: Vec<UsdtExchange> = Vec::new();

    for transaction in transaction_list.transactions {
        substreams::log::println("-------------1");
        substreams::log::println(transaction.messages.len().to_string());
        //substreams::log::println(transaction.result_log);
        for message in transaction.messages {
            substreams::log::println("-------------2");
            if let Some(usdt_exchange) = handle_msg_execute_contract(&message, &transaction.result_events) {
                usdt_exchanges.push(usdt_exchange)
            }
        }
    }

    Ok(UsdtExchangeList {
        exchanges: usdt_exchanges,
    })
}

fn handle_msg_execute_contract(message: &Message, events: &Vec<Event>) -> Option<UsdtExchange> {
    if let Some(value) = &message.value {
        substreams::log::println("-------------");
        match value {
            // The value is not one of the known types provided by the "map_transactions" module
            // You must use the "Other" type and decode the data if needed.
            Value::Other(message) => {
                if message.type_url != "/cosmwasm.wasm.v1.MsgExecuteContract"
                    && message.type_url != "/injective.wasmx.v1.MsgExecuteContractCompat"
                {
                    return None;
                }

                if !is_dojo_smart_contract(events) {
                    return None;
                }

                if let Some(event) = events.iter().find(|event| event.r#type == "wasm") {
                    return extract_data_from_event(event);
                }
            }
            _ => {
                return None;
            }
        }
    }

    return None;
}

fn is_dojo_smart_contract(events: &Vec<Event>) -> bool {
    if let Some(event) = events.iter().find(|event| event.r#type == "execute") {
        if let Some(attr) = event.attributes.iter().find(|attr| attr.key == "_contract_address") {
            return attr.value == DOJO_SMART_CONTRACT;
        }
    }

    return false;
}

fn extract_data_from_event(event: &Event) -> Option<UsdtExchange> {
    let mut offer_asset = &String::new();
    let mut offer_amount = &String::new();
    let mut ask_asset = &String::new();
    let mut ask_amount = &String::new();

    event.attributes.iter().for_each(|att| {
        if att.key == "offer_asset" {
            offer_asset = &att.value;
        }

        if att.key == "offer_amount" {
            offer_amount = &att.value;
        }

        if att.key == "ask_asset" {
            ask_asset = &att.value;
        }

        if att.key == "ask_amount" || att.key == "return_amount" {
            ask_amount = &att.value;
        }
    });

    if ask_asset == USDT_ADDRESS && !ask_amount.is_empty() {
        return Some(UsdtExchange {
            amount: ask_amount.to_string(),
        });
    }

    if offer_asset == USDT_ADDRESS && !offer_amount.is_empty() {
        return Some(UsdtExchange {
            amount: offer_amount.to_string(),
        });
    }

    return None;
}
