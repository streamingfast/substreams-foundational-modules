mod pb;

use crate::pb::sf::substreams::foundational_store::v1::{Entries, Entry};
use crate::pb::sf::substreams::solana::spl::v1::{AccountOwner, InitializedAccount};

use crate::pb::sol::transactions::v1::Transactions as SolanaTransactions;
use prost::Message;
use prost_types::Any;
use substreams::errors::Error;
use substreams_solana::block_view::InstructionView;
use substreams_solana::pb::sf::solana::r#type::v1::{ConfirmedTransaction, TransactionStatusMeta};

use substreams_solana_program_instructions::token_instruction_2022::TokenInstruction;

pub const SOLANA_TOKEN_PROGRAM_KEG: &str = "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA";
pub const SOLANA_TOKEN_PROGRAM_ZQB: &str = "TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb";

struct OutputInstructions {
    transaction_hash: String,
    ordinal: i64,
    initialized_accounts: Vec<InitializedAccount>,
}

impl OutputInstructions {
    pub fn new(transaction_hash: String) -> Self {
        Self {
            transaction_hash,
            ordinal: 0,
            initialized_accounts: vec![],
        }
    }

    pub fn add(&mut self, item: InitializedAccount) {
        self.initialized_accounts.push(item);

        self.ordinal += 1;
    }
}

#[substreams::handlers::map]
fn map_spl_initialized_account(_params: String, transactions: SolanaTransactions) -> Result<Entries, Error> {
    let mut initialized_accounts: Vec<InitializedAccount> = vec![];
    for confirmed_trx in transactions_owned(transactions) {
        let hash = bs58::encode(confirmed_trx.hash()).into_string();

        let mut output_instructions = OutputInstructions::new(hash.clone());

        for instruction in confirmed_trx.walk_instructions() {
            process_instruction(&mut output_instructions, &instruction);
        }

        initialized_accounts.extend(output_instructions.initialized_accounts);
    }

    let mut entries: Vec<Entry> = vec![];
    for initialized_account in initialized_accounts.iter() {
        let account = &initialized_account.account;
        let account_owner = AccountOwner {
            mint_address: bs58::decode(&initialized_account.mint_address).into_vec().unwrap(),
            owner: bs58::decode(&initialized_account.owner).into_vec().unwrap(),
        };

        let mut buf = Vec::new();
        prost::Message::encode(&account_owner, &mut buf).unwrap();

        let any = Any {
            type_url: "type.googleapis.com/sf.substreams.solana.spl.v1.AccountOwner".to_string(),
            value: buf,
        };

        let entry = Entry {
            key: bs58::decode(account).into_vec().unwrap(),
            value: Some(any),
        };
        // substreams::log::info!("adding key: {}", account);
        entries.push(entry);
    }

    Ok(Entries { entries })
}

/// Iterates over successful transactions in given block and take ownership.
fn transactions_owned(transactions: SolanaTransactions) -> impl Iterator<Item = ConfirmedTransaction> {
    transactions.transactions.into_iter().filter_map(|trx| -> Option<ConfirmedTransaction> {
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

fn process_instruction(output: &mut OutputInstructions, compile_instruction: &InstructionView) {
    let trx_hash = &bs58::encode(compile_instruction.transaction().hash()).into_string();
    match compile_instruction.program_id().to_string().as_ref() {
        SOLANA_TOKEN_PROGRAM_KEG | SOLANA_TOKEN_PROGRAM_ZQB => {
            match process_token_instruction(output, compile_instruction, compile_instruction.meta()) {
                Err(err) => {
                    substreams::log::info!("Skipping unknown token instruction in tx {}: {}", trx_hash, err);
                }
                _ => {}
            }
        }
        _ => {}
    }
}


fn process_token_instruction(
    output: &mut OutputInstructions,
    instruction: &InstructionView,
    _meta: &substreams_solana::pb::sf::solana::r#type::v1::TransactionStatusMeta,
) -> Result<(), Error> {
    match TokenInstruction::unpack(&instruction.data()) {
        Err(err) => {
            return Err(anyhow::anyhow!("unpacking token instruction: {}", err));
        }
        Ok(token_instruction) => match token_instruction {
            TokenInstruction::InitializeAccount {} => {
                let mint = &instruction.accounts()[1];

                let account = &instruction.accounts()[0];
                let owner = &instruction.accounts()[2];

                output.add(InitializedAccount {
                    account: account.to_string(),
                    mint_address: mint.to_string(),
                    owner: owner.to_string(),
                });
            }
            TokenInstruction::InitializeAccount2 { owner: ow } | TokenInstruction::InitializeAccount3 { owner: ow } => {
                let mint = &instruction.accounts()[1];

                let account = &instruction.accounts()[0];

                output.add(InitializedAccount {
                    account: account.to_string(),
                    mint_address: mint.to_string(),
                    owner: bs58::encode(ow).into_string(),
                });
            }
            _ => {}
        },
    }

    Ok(())
}
