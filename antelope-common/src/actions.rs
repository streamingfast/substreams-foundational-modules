use substreams_antelope::pb::{ActionTrace, PermissionLevel};

// https://docs.dfuse.eosnation.io/eosio/public-apis/reference/search/terms/
pub fn action_keys(action: &ActionTrace) -> Vec<String> {
    let receiver = &action.receiver;
    let action = action.action.as_ref().unwrap();
    let mut keys = Vec::with_capacity(action.authorization.len() * 2 + 3);

    keys.extend(vec![
        format!("code:{}", action.account),
        format!("receiver:{}", receiver),
        format!("action:{}", action.name),
    ]);

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

    keys
}
