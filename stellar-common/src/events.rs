use core::panic;

use stellar_xdr::curr::{
    ContractEventBody, ContractEventType, ContractEventV0, Hash, TransactionMeta,
};
use substreams::Hex;

use crate::{
    index::event_keys,
    pb::sf::substreams::stellar::r#type::v1::{Event, Events, Transactions},
    utils::decode_transaction_meta,
};

#[substreams::handlers::map]
fn map_events(transactions: Transactions) -> Result<Events, substreams::errors::Error> {
    let contract_events = transactions
        .transactions
        .into_iter()
        .filter_map(|transaction| {
            let meta = match decode_transaction_meta(&transaction.result_meta_xdr) {
                Ok(meta) => meta,
                Err(e) => {
                    substreams::log::info!("Skipping transaction: decode error: {}", e);
                    return None;
                }
            };

            match meta {
                TransactionMeta::V3(v3) => v3.soroban_meta.map(|soroban_meta| soroban_meta.events),
                _ => None,
            }
        })
        .flat_map(|events| {
            events
                .iter()
                .map(|event| {
                    let body = match &event.body {
                        ContractEventBody::V0(contract_event_v0) => contract_event_v0,
                    };

                    Event {
                        contract_id: map_contract_id(&event.contract_id),
                        r#type: map_event_type(event.type_),
                        topics: map_event_topics(&body),
                        data: map_event_data(&body),
                    }
                })
                .collect::<Vec<Event>>()
        })
        .collect();

    Ok(Events {
        events: contract_events,
    })
}

#[substreams::handlers::map]
fn filtered_events(query: String, events: Events) -> Result<Events, substreams::errors::Error> {
    let query = substreams::expr_matcher(&query);

    let filtered_events: Vec<Event> = events
        .events
        .into_iter()
        .filter(|event| {
            let keys = event_keys(event);
            query.matches_keys(&keys)
        })
        .collect();

    Ok(Events {
        events: filtered_events,
    })
}

fn map_contract_id(contract_id: &Option<Hash>) -> Option<String> {
    return match contract_id {
        Some(hash) => Some(Hex::encode(hash)),
        _ => None,
    };
}

fn map_event_type(event_type: ContractEventType) -> String {
    return match event_type {
        ContractEventType::Diagnostic => String::from("Diagnostic"),
        ContractEventType::System => String::from("System"),
        ContractEventType::Contract => String::from("Contract"),
    };
}

fn map_event_topics(body: &ContractEventV0) -> Vec<String> {
    body.topics
        .iter()
        .filter_map(|topic| serde_json::to_string(topic).ok())
        .collect()
}

fn map_event_data(body: &ContractEventV0) -> String {
    match serde_json::to_string(&body.data) {
        Ok(s) => s,
        Err(_) => String::new()
    }
}
