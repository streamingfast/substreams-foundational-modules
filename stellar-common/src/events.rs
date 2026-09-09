use std::io::Cursor;
use stellar_xdr::{
    ContractEvent as XdrContractEvent, ContractEventBody, ContractEventType, ContractEventV0,
    ContractId, Limited, Limits, ReadXdr,
};
use substreams::Hex;

use crate::{
    index::event_keys,
    pb::sf::{
        stellar::r#type::v1::Block,
        substreams::stellar::r#type::v1::{Event, Events},
    },
    utils::transaction_failed,
};

#[substreams::handlers::map]
fn map_events(block: Block) -> Result<Events, substreams::errors::Error> {
    let contract_events = block
        .transactions
        .into_iter()
        .filter(|transaction| !transaction_failed(transaction.status.to_i32()))
        .filter_map(|transaction| transaction.events.into_option())
        .flat_map(|events| {
            events
                .contract_events_xdr
                .into_iter()
                .flat_map(|contract_event_group| {
                    contract_event_group
                        .events
                        .into_iter()
                        .filter_map(|event_bytes| {
                            match decode_contract_event(&event_bytes) {
                                Ok(event) => Some(event),
                                Err(e) => {
                                    substreams::log::info!("Skipping event: decode error: {}", e);
                                    None
                                }
                            }
                        })
                        .collect::<Vec<Event>>()
                })
                .collect::<Vec<Event>>()
        })
        .collect();

    Ok(Events {
        events: contract_events,
    })
}

fn decode_contract_event(bytes: &[u8]) -> Result<Event, stellar_xdr::Error> {
    let buf = Cursor::new(bytes);
    let event = XdrContractEvent::read_xdr(&mut Limited::new(buf, Limits::none()))?;

    let body = match &event.body {
        ContractEventBody::V0(v0) => v0,
    };

    Ok(Event {
        contract_id: map_contract_id(&event.contract_id),
        r#type: map_event_type(event.type_),
        topics: map_event_topics(body),
        data: map_event_data(body),
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
