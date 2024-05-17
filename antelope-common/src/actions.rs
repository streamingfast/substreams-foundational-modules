use substreams_antelope::pb::ActionTrace;

pub fn action_keys(action: &ActionTrace) -> Vec<String> {
    let auths = action.receipt.as_ref().unwrap().auth_sequence.iter();
    let mut keys = Vec::with_capacity(auths.len() + 3);

    keys.extend(vec![
        format!("code:{}", action.action.as_ref().unwrap().account),
        format!("receiver:{}", action.receiver),
        format!("action:{}", action.action.as_ref().unwrap().name),
    ]);

    keys.extend(auths.map(|auth| format!("auth:{}", auth.account_name)));

    keys
}
