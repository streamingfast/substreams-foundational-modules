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
    let mut proto_events = Events::default();

    let contract_events: Vec<Event> = transactions
        .transactions
        .iter()
        .flat_map(|transaction| {
            // Skip the transaction is it's not possible to decode the TransactionMeta
            let iter: Box<dyn Iterator<Item = Event>> =
                match decode_transaction_meta(&transaction.result_meta_xdr) {
                    Ok(result_meta) => match result_meta {
                        TransactionMeta::V3(v3) => match v3.soroban_meta {
                            Some(soroban_meta) => {
                                // Collect events into Vec to own the data
                                let events: Vec<Event> = soroban_meta
                                    .events
                                    .iter()
                                    .map(|event| {
                                        let body = get_event_body(&event.body);
                                        Event {
                                            contract_id: map_contract_id(&event.contract_id),
                                            r#type: map_event_type(event.type_),
                                            topics: map_event_topics(&body),
                                            data: map_event_data(&body),
                                        }
                                    })
                                    .collect();

                                Box::new(events.into_iter())
                            }
                            None => Box::new(std::iter::empty()),
                        },
                        _ => Box::new(std::iter::empty()),
                    },
                    Err(e) => {
                        substreams::log::info!("Skipping transaction: decode error: {}", e);
                        Box::new(std::iter::empty())
                    }
                };
            iter
        })
        .collect();

    proto_events.events = contract_events;

    Ok(proto_events)
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

fn get_event_body(body: &ContractEventBody) -> ContractEventV0 {
    return match body {
        ContractEventBody::V0(v0) => v0.clone(),
        _ => {
            panic!("ContractEventBody is not of type V0")
        }
    };
}

fn map_event_topics(body: &ContractEventV0) -> Vec<String> {
    return body
        .topics
        .iter()
        .map(|topic| {
            return serde_json::to_string(topic).unwrap();
        })
        .collect();
}

fn map_event_data(body: &ContractEventV0) -> String {
    return serde_json::to_string(&body.data).unwrap();
}
