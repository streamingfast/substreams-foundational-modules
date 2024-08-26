use crate::pb::sol::instructions::v1::Instruction;
use crate::pb::sol::instructions::v1::Instructions;
use crate::pb::sol::transactions::v1::Transactions;
use substreams::pb::sf::substreams::index::v1::Keys;
use substreams_solana::Address;

#[substreams::handlers::map]
fn all_instructions_without_votes(transactions: Transactions) -> Result<Instructions, substreams::errors::Error> {
    Ok(Instructions {
        instructions: transactions
            .iter()
            .flat_map(|trx| {
                trx.instructions()
                    .map(|view: substreams_solana::block_view::InstructionView| Instruction {
                        program_id: view.program_id().to_string(),
                        data: view.data().clone(),
                        accounts: view.accounts().iter().map(Address::to_string).collect(),
                        tx_hash: trx.id(),
                    })
            })
            .collect(),
    })
}

#[substreams::handlers::map]
fn index_instructions_without_votes(instructions: Instructions) -> Result<Keys, substreams::errors::Error> {
    let keys: Vec<String> = instructions
        .instructions
        .iter()
        .map(|inst| format!("program:{}", inst.program_id))
        .collect();

    Ok(Keys { keys })
}
