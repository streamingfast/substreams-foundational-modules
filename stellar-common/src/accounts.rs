use crate::{
    constants,
    pb::sf::{
        stellar::r#type::v1::Block,
        substreams::stellar::r#type::v1::{Account, Accounts},
    },
    utils,
};
use core::panic;
use substreams::Hex;

#[substreams::handlers::map]
fn map_created_accounts(block: Block) -> Result<Accounts, substreams::errors::Error> {
    let mut accounts = Accounts::default();

    block.transactions.iter().for_each(|transaction| {
        let hash = Hex(&transaction.hash).to_string();
        if utils::transaction_failed(transaction.status) {
            return;
        }

        let decoded_transaction = match utils::decode_transaction(&transaction.result_xdr, &transaction.envelope_xdr) {
            Ok(trx) => trx,
            Err(_) => return,
        };

        decoded_transaction
            .operations
            .iter()
            .for_each(|operation| match &operation.body {
                stellar_xdr::curr::OperationBody::CreateAccount(create_account) => {
                    accounts.accounts.push(Account {
                        trx_hash: hash.clone(),
                        address: create_account.destination.to_string(),
                        balance: (create_account.starting_balance as f64) / constants::XLM_DENOMINATOR,
                    });
                }
                _ => {}
            });
    });

    Ok(accounts)
}

#[substreams::handlers::map]
fn map_deleted_accounts(block: Block) -> Result<Accounts, substreams::errors::Error> {
    let mut accounts = Accounts::default();

    block.transactions.iter().for_each(|transaction| {
        let hash = Hex(&transaction.hash).to_string();
        if utils::transaction_failed(transaction.status) {
            return;
        }

        let decoded_transaction_meta = match utils::decode_transaction_meta(&transaction.result_meta_xdr) {
            Ok(trx) => trx,
            Err(_) => return,
        };

        for operation_meta in decoded_transaction_meta.operations.as_vec() {
            for change in operation_meta.changes.0.as_vec() {
                match change {
                    // While this may be a way to determine deleted accounts,
                    // parsing out the AccountMerge operation is more
                    // straightforward. It's what we do in Horizon [1] to
                    // generate an "AccountDeleted" side-effect.
                    //
                    // [1]: https://github.com/stellar/go/blob/e30ccf40d186845f2d3332a9ae9a0d01da9dcf7f/services/horizon/internal/ingest/processors/effects_processor.go#L845-L865
                    stellar_xdr::curr::LedgerEntryChange::Removed(ledger_key) => match ledger_key {
                        stellar_xdr::curr::LedgerKey::Account(ledger_key_account) => {
                            accounts.accounts.push(Account {
                                trx_hash: hash.clone(),
                                address: ledger_key_account.account_id.to_string(),
                                balance: 0.0,
                            });
                        }
                        _ => {}
                    },
                    _ => {}
                }
            }
        }
    });

    Ok(accounts)
}
