use crate::pb::sf::solana::r#type::v1::AccountBlockLazyView;
use crate::pb::sf::substreams::solana::r#type::v1::FilteredAccounts;
use buffa::view::LazyMessageView;
use core::str;
use std::collections::HashSet;
use std::convert::TryInto;
use substreams::pb::sf::substreams::index::v1::Keys;

#[substreams::handlers::map]
fn index_accounts(
    account_block: &AccountBlockLazyView<'_>,
) -> Result<Keys, substreams::errors::Error> {
    let mut keys = Keys::default();
    let mut seen_accounts = HashSet::new();
    let mut seen_owners = HashSet::new();
    let mut buf = [0u8; 44];

    for account in account_block.accounts.iter() {
        let account = account?;

        if seen_accounts.insert(account.address) {
            let len = five8::encode_32(to_32_array(account.address), &mut buf);
            let key =
                "account:".to_string() + unsafe { str::from_utf8_unchecked(&buf[..len as usize]) };
            keys.keys.push(key);
        }

        if seen_owners.insert(account.owner) {
            let len = five8::encode_32(to_32_array(account.owner), &mut buf);
            let key =
                "owner:".to_string() + unsafe { str::from_utf8_unchecked(&buf[..len as usize]) };
            keys.keys.push(key);
        }
    }

    Ok(keys)
}

#[substreams::handlers::map]
fn filtered_accounts(
    query: String,
    account_block: &AccountBlockLazyView<'_>,
) -> Result<FilteredAccounts, substreams::errors::Error> {
    let expr_matcher = substreams::sqe::expr_matcher(&query);

    let mut accounts = Vec::new();
    let mut buf = [0u8; 44];

    for account in account_block.accounts.iter() {
        let account = account?;

        let len = five8::encode_32(to_32_array(account.address), &mut buf);
        let acc_key =
            "account:".to_string() + unsafe { str::from_utf8_unchecked(&buf[..len as usize]) };
        let len = five8::encode_32(to_32_array(account.owner), &mut buf);
        let owner_key =
            "owner:".to_string() + unsafe { str::from_utf8_unchecked(&buf[..len as usize]) };

        if expr_matcher.matches_keys(&[acc_key, owner_key]) {
            accounts.push(account.to_owned_message()?);
        }
    }

    Ok(FilteredAccounts { accounts })
}

fn to_32_array(slice: &[u8]) -> &[u8; 32] {
    slice.try_into().expect("slice with incorrect length")
}
