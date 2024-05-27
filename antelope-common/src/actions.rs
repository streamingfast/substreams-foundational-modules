use serde_json::Value;
use substreams_antelope::pb::{ActionTrace, PermissionLevel};

// i.e. https://docs.dfuse.eosnation.io/eosio/public-apis/reference/search/terms/
pub fn action_keys(trace: &ActionTrace) -> Vec<String> {
    let action = trace.action.as_ref().unwrap();
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
