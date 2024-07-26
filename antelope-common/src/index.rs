use std::collections::HashSet;

use serde_json::Value;
use substreams::pb::sf::substreams::index::v1::Keys;
use substreams_antelope::pb::{ActionTrace, ActionTraces, PermissionLevel};

#[substreams::handlers::map]
fn index_actions(actions: ActionTraces) -> Result<Keys, substreams::errors::Error> {
    collect_keys(actions, action_keys)
}

#[substreams::handlers::map]
fn index_actions_extra(actions: ActionTraces) -> Result<Keys, substreams::errors::Error> {
    collect_keys(actions, action_keys_extra)
}

fn collect_keys<F>(
    actions: ActionTraces,
    key_extractor: F,
) -> Result<Keys, substreams::errors::Error>
where
    F: Fn(&ActionTrace) -> Vec<String>,
{
    let keys = actions
        .action_traces
        .iter()
        .flat_map(key_extractor)
        .collect::<HashSet<_>>()
        .into_iter()
        .collect();

    Ok(Keys { keys })
}

// i.e. https://docs.dfuse.eosnation.io/eosio/public-apis/reference/search/terms/
pub fn action_keys_extra(trace: &ActionTrace) -> Vec<String> {
    let action = trace.action.as_ref().unwrap();
    if action.account == "eosio" && action.name == "onblock" {
        return vec![];
    }
    let mut keys = Vec::with_capacity(action.authorization.len() * 2 + 5 + 3);
    let json_data: Value = match serde_json::from_str(&action.json_data) {
        Ok(data) => data,
        Err(_) => Value::Object(Default::default()),
    };

    keys.extend(vec![
        format!("code:{}", action.account),
        format!("receiver:{}", trace.receiver),
        format!("action:{}", action.name),
    ]);

    keys.extend(
        json_data
            .as_object()
            .unwrap()
            .iter()
            .filter_map(|(key, value)| match value {
                Value::String(value) => Some(format!("data.{}:{}", key, value)),
                Value::Number(value) => Some(format!("data.{}:{}", key, value)),
                Value::Bool(value) => Some(format!("data.{}:{}", key, value)),
                _ => None,
            }),
    );

    keys.extend(
        action
            .authorization
            .iter()
            .flat_map(|PermissionLevel { actor, permission }| {
                vec![
                    format!("auth:{actor}@{permission}"),
                    format!("auth:{actor}"),
                ]
            }),
    );

    if trace.creator_action_ordinal == 0 {
        keys.push("input:true".to_string());
    }

    keys
}

pub fn action_keys(trace: &ActionTrace) -> Vec<String> {
    let action = trace.action.as_ref().unwrap();
    if action.account == "eosio" && action.name == "onblock" {
        return vec![];
    }
    vec![
        format!("code:{}", action.account),
        format!("receiver:{}", trace.receiver),
        format!("action:{}", action.name),
    ]
}
