mod pb;
mod test;

use crate::pb::sf::substreams::foundational_store::v1::{Entries, Entry};
use crate::pb::sf::substreams::solana::spl::v1::AccountOwner;

use borsh::BorshDeserialize;

use crate::pb::sol::transactions::v1::Transactions as SolanaTransactions;
use prost::Message;
use prost_types::Any;
use spl_token_2022::instruction::TokenInstruction;
use spl_token_metadata_interface::instruction::TokenMetadataInstruction;
use substreams::errors::Error;
use substreams_solana::block_view::InstructionView;
use substreams_solana::pb::sf::solana::r#type::v1::ConfirmedTransaction;

pub const SOLANA_TOKEN_PROGRAM_KEG: &str = "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA";
pub const SOLANA_TOKEN_PROGRAM_ZQB: &str = "TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb";

#[substreams::handlers::map]
fn map_spl_initialized_account(
    _params: String,
    transactions: SolanaTransactions,
) -> Result<Entries, Error> {
    _map_spl_initialized_account(transactions)
}

pub fn _map_spl_initialized_account(transactions: SolanaTransactions) -> Result<Entries, Error> {
    let mut initialized_accounts: Vec<InitializedAccountEntry> = vec![];
    for confirmed_trx in transactions_owned(transactions) {
        for instruction in confirmed_trx.walk_instructions() {
            process_instruction(&mut initialized_accounts, &instruction);
        }
    }

    let mut entries: Vec<Entry> = Vec::with_capacity(initialized_accounts.len());
    for initialized_account in initialized_accounts.into_iter() {
        let account = initialized_account.account;
        let account_owner = AccountOwner {
            mint_address: initialized_account.mint_address,
            owner: initialized_account.owner,
        };

        let mut buf = Vec::new();
        prost::Message::encode(&account_owner, &mut buf).unwrap();

        let entry = Entry {
            key: account,
            value: Some(Any {
                type_url: "type.googleapis.com/sf.substreams.solana.spl.v1.AccountOwner"
                    .to_string(),
                value: buf,
            }),
        };
        // substreams::log::info!("adding key: {}", account);
        entries.push(entry);
    }

    Ok(Entries { entries })
}

/// Iterates over successful transactions in given block and take ownership.
fn transactions_owned(
    transactions: SolanaTransactions,
) -> impl Iterator<Item = ConfirmedTransaction> {
    transactions
        .transactions
        .into_iter()
        .filter_map(|trx| -> Option<ConfirmedTransaction> {
            if let Some(meta) = &trx.meta {
                if meta.err.is_none() {
                    // Convert between protobuf types by serializing and deserializing
                    let mut buf = Vec::new();
                    if Message::encode(&trx, &mut buf).is_ok() {
                        if let Ok(converted) = ConfirmedTransaction::decode(&buf[..]) {
                            return Some(converted);
                        }
                    }
                }
            }
            None
        })
}

fn process_instruction(
    initialized_accounts: &mut Vec<InitializedAccountEntry>,
    compile_instruction: &InstructionView,
) {
    match compile_instruction.program_id().to_string().as_ref() {
        SOLANA_TOKEN_PROGRAM_KEG | SOLANA_TOKEN_PROGRAM_ZQB => {
            match process_token_instruction(
                initialized_accounts,
                compile_instruction,
                compile_instruction.meta(),
            ) {
                Err(err) => {
                    let trx_hash =
                        &bs58::encode(compile_instruction.transaction().hash()).into_string();

                    substreams::log::info!(
                        "Skipping unknown token instruction in tx {}: {}",
                        trx_hash,
                        err
                    );
                }
                _ => {}
            }
        }
        _ => {}
    }
}

fn process_token_instruction(
    initialized_accounts: &mut Vec<InitializedAccountEntry>,
    instruction: &InstructionView,
    _meta: &substreams_solana::pb::sf::solana::r#type::v1::TransactionStatusMeta,
) -> Result<(), Error> {

    if instruction.data()[0] > 44 {
        match TokenMetadataInstruction::unpack(instruction.data().as_slice()) {
            Ok(_) => {
                substreams::log::info!("Skipping metadata instruction");
                return Ok(());
            }
            Err(_) => {}
        }
    }

    match TokenInstruction::unpack(&instruction.data()) {
        Err(err) => {
            panic!("unpacking token instruction: {}", err);
        }
        Ok(token_instruction) => match token_instruction {
            TokenInstruction::InitializeAccount {} => {
                let accounts = instruction.accounts();

                let mint = &accounts[1];
                let account = &accounts[0];
                let owner = &accounts[2];

                initialized_accounts.push(InitializedAccountEntry {
                    account: account.0.clone(),
                    mint_address: mint.0.clone(),
                    owner: owner.0.clone(),
                });
            }
            TokenInstruction::InitializeAccount2 { owner: ow }
            | TokenInstruction::InitializeAccount3 { owner: ow } => {
                let accounts = instruction.accounts();

                let mint = &accounts[1];
                let account = &accounts[0];

                initialized_accounts.push(InitializedAccountEntry {
                    account: account.0.clone(),
                    mint_address: mint.0.clone(),
                    owner: ow.to_bytes().into(),
                });
            }
            _ => {}
        },
    }

    Ok(())
}

struct InitializedAccountEntry {
    pub account: Vec<u8>,
    pub mint_address: Vec<u8>,
    pub owner: Vec<u8>,
}
