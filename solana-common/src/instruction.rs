use crate::pb::sol::instructions::v1::Instruction;
use crate::pb::sol::instructions::v1::Instructions;
use crate::pb::sol::transactions::v1::Transactions;
use substreams::pb::sf::substreams::index::v1::Keys;
use substreams::matches_keys_in_parsed_expr;
use substreams_solana::pb::sf::solana::r#type::v1::ConfirmedTransaction;

#[substreams::handlers::map]
fn all_instructions(transactions: Transactions) -> Result<Instructions, substreams::errors::Error> {
    let instructions = get_instructions_from_transactions(&transactions.transactions);

    Ok(Instructions {
        instructions,
    })
}

#[substreams::handlers::map]
fn index_instructions(instructions: Instructions) -> Result<Keys, substreams::errors::Error> {
    let keys: Vec<String> = instructions.instructions
            .into_iter()
            .map(|inst| format!("program:{}", inst.program_id))
            .collect();

    Ok(Keys { keys })
}

#[substreams::handlers::map]
fn filtered_instructions(query: String, instructions: Instructions) -> Result<Instructions, substreams::errors::Error> {
    let filtered_instructions = instructions.instructions
            .into_iter()
            .filter(|inst| {
                let mut keys = Vec::new();
                keys.push(format!("program:{}", inst.program_id));

                matches_keys_in_parsed_expr(&keys, &query).expect("matching events from query")
            }).collect();

    Ok(Instructions {
        instructions: filtered_instructions
    })
}

pub fn get_instructions_from_transactions(transactions: &Vec<ConfirmedTransaction>) -> Vec<Instruction> {
    let mut processed_instructions: Vec<Instruction> = Vec::new();

    for confirmed_transaction in transactions.into_iter() {
        let confirmed_borrowed = &confirmed_transaction;
        let acct_keys: Vec<&Vec<u8>> = confirmed_borrowed.resolved_accounts();
        
        if let Some(transaction) = &confirmed_borrowed.transaction {
            for instruction in confirmed_transaction.all_instructions().into_iter() {
                let general_instruction = instruction.instruction;
                processed_instructions.push(Instruction {
                    program_id: bs58::encode(acct_keys[general_instruction.program_id_index() as usize].clone()).into_string(),
                    data: bs58::encode(&general_instruction.data()).into_string(),
                    accounts: general_instruction
                        .accounts()
                        .iter()
                        .map(|acct| bs58::encode(acct_keys[*acct as usize].clone()).into_string())
                        .collect(),
                    tx_hash: bs58::encode(transaction.signatures[0].clone()).into_string(),
                });
            }
        }
    }

    return processed_instructions;
}
