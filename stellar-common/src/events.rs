use stellar_xdr::curr::{
    ContractEventBody, ContractEventType, ContractEventV0, ContractId, TransactionMeta,
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
    let query = substreams::sqe::expr_matcher(&query);

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

fn map_contract_id(contract_id: &Option<ContractId>) -> Option<String> {
    return match contract_id {
        Some(contract_id) => Some(Hex::encode(contract_id.0.clone())),
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
        Err(_) => String::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pb::sf::substreams::stellar::r#type::v1::Event;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_event_keys_with_contract_id() {
        let event = Event {
            r#type: "Contract".to_string(),
            contract_id: Some("abcd1234".to_string()),
            topics: vec![],
            data: "".to_string(),
        };

        let keys = event_keys(&event);

        assert_eq!(keys.len(), 2);
        assert!(keys.contains(&"type:Contract".to_string()));
        assert!(keys.contains(&"contract_id:abcd1234".to_string()));
    }

    #[test]
    fn test_event_keys_without_contract_id() {
        let event = Event {
            r#type: "System".to_string(),
            contract_id: None,
            topics: vec![],
            data: "".to_string(),
        };

        let keys = event_keys(&event);

        assert_eq!(keys.len(), 1);
        assert!(keys.contains(&"type:System".to_string()));
    }

    #[test]
    fn test_event_keys_diagnostic_type() {
        let event = Event {
            r#type: "Diagnostic".to_string(),
            contract_id: Some("xyz789".to_string()),
            topics: vec!["topic1".to_string(), "topic2".to_string()],
            data: "some data".to_string(),
        };

        let keys = event_keys(&event);

        assert_eq!(keys.len(), 2);
        assert!(keys.contains(&"type:Diagnostic".to_string()));
        assert!(keys.contains(&"contract_id:xyz789".to_string()));
    }

    #[test]
    fn test_event_keys_empty_contract_id() {
        let event = Event {
            r#type: "Contract".to_string(),
            contract_id: Some("".to_string()),
            topics: vec![],
            data: "".to_string(),
        };

        let keys = event_keys(&event);

        assert_eq!(keys.len(), 2);
        assert!(keys.contains(&"type:Contract".to_string()));
        assert!(keys.contains(&"contract_id:".to_string()));
    }

    #[test]
    fn test_event_keys_with_topics() {
        let event = Event {
            r#type: "Contract".to_string(),
            contract_id: Some("test_contract".to_string()),
            topics: vec![
                "{\"symbol\":\"transfer\"}".to_string(),
                "{\"address\":\"CB7FKGSTHP75ORTIZGGMVUTQLEMVTSEOI4QORQPCABJSGTAATDFCE2YV\"}"
                    .to_string(),
                "{\"address\":\"CB3JAPDEIMA3OOSALUHLYRGM2QTXGVD3EASALPFMVEU2POLLULJBT2XN\"}"
                    .to_string(),
                "{\"string\":\"USDC:GA5ZSEJYB37JRC5AVCIA5MOP4RHTM335X2KGX3IHOJAPP5RE34K4KZVN\"}"
                    .to_string(),
            ],
            data: "".to_string(),
        };

        let keys = event_keys(&event);

        assert_eq!(keys.len(), 6);
        assert!(keys.contains(&"type:Contract".to_string()));
        assert!(keys.contains(&"contract_id:test_contract".to_string()));
        assert!(keys.contains(&"topic:symbol:transfer".to_string()));
        assert!(keys.contains(
            &"topic:address:CB7FKGSTHP75ORTIZGGMVUTQLEMVTSEOI4QORQPCABJSGTAATDFCE2YV".to_string()
        ));
        assert!(keys.contains(
            &"topic:address:CB3JAPDEIMA3OOSALUHLYRGM2QTXGVD3EASALPFMVEU2POLLULJBT2XN".to_string()
        ));
        assert!(keys.contains(
            &"topic:string:USDC:GA5ZSEJYB37JRC5AVCIA5MOP4RHTM335X2KGX3IHOJAPP5RE34K4KZVN"
                .to_string()
        ));
    }

    #[test]
    fn test_event_keys_with_invalid_topic_json() {
        let event = Event {
            r#type: "Contract".to_string(),
            contract_id: Some("test_contract".to_string()),
            topics: vec![
                "{\"symbol\":\"transfer\"}".to_string(),
                "invalid json".to_string(),
                "{\"address\":\"CB7FKGSTHP75ORTIZGGMVUTQLEMVTSEOI4QORQPCABJSGTAATDFCE2YV\"}"
                    .to_string(),
            ],
            data: "".to_string(),
        };

        let keys = event_keys(&event);

        assert_eq!(keys.len(), 4);
        assert!(keys.contains(&"type:Contract".to_string()));
        assert!(keys.contains(&"contract_id:test_contract".to_string()));
        assert!(keys.contains(&"topic:symbol:transfer".to_string()));
        assert!(keys.contains(
            &"topic:address:CB7FKGSTHP75ORTIZGGMVUTQLEMVTSEOI4QORQPCABJSGTAATDFCE2YV".to_string()
        ));
    }

    #[test]
    fn test_event_keys_with_non_string_topic_values() {
        let event = Event {
            r#type: "Contract".to_string(),
            contract_id: Some("test_contract".to_string()),
            topics: vec![
                "{\"symbol\":\"transfer\"}".to_string(),
                "{\"number\":123}".to_string(),
                "{\"boolean\":true}".to_string(),
                "{\"array\":[1,2,3]}".to_string(),
            ],
            data: "".to_string(),
        };

        let keys = event_keys(&event);

        assert_eq!(keys.len(), 3);
        assert!(keys.contains(&"type:Contract".to_string()));
        assert!(keys.contains(&"contract_id:test_contract".to_string()));
        assert!(keys.contains(&"topic:symbol:transfer".to_string()));
    }

    #[test]
    fn test_event_keys_with_empty_topics() {
        let event = Event {
            r#type: "System".to_string(),
            contract_id: None,
            topics: vec![],
            data: "".to_string(),
        };

        let keys = event_keys(&event);

        assert_eq!(keys.len(), 1);
        assert!(keys.contains(&"type:System".to_string()));
    }
}
