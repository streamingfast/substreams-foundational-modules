mod pb;

use crate::pb::cosmos::authz::v1beta1::MsgExec;
use crate::pb::cosmos::bank::v1beta1::MsgMultiSend;
use crate::pb::cosmos::bank::v1beta1::MsgSend;
use crate::pb::cosmos::crisis::v1beta1::MsgVerifyInvariant;
use crate::pb::cosmos::distribution::v1beta1::MsgFundCommunityPool;
use crate::pb::cosmos::distribution::v1beta1::MsgSetWithdrawAddress;
use crate::pb::cosmos::distribution::v1beta1::MsgWithdrawDelegatorReward;
use crate::pb::cosmos::distribution::v1beta1::MsgWithdrawValidatorCommission;
use crate::pb::cosmos::evidence::v1beta1::MsgSubmitEvidence;
use crate::pb::cosmos::gov::v1beta1::MsgDeposit;
use crate::pb::cosmos::gov::v1beta1::MsgSubmitProposal;
use crate::pb::cosmos::gov::v1beta1::MsgVote;
use crate::pb::cosmos::slashing::v1beta1::MsgUnjail;
use crate::pb::cosmos::tx::v1beta1::Tx;
use crate::pb::sf::cosmos::r#type::v2::Block;
use crate::pb::spkg::cosmos::v1::transaction::Message;
use anyhow::anyhow;
use pb::spkg::cosmos::v1::transaction::message::Value;
use pb::spkg::cosmos::v1::Transaction;
use pb::spkg::cosmos::v1::TransactionList;
use prost_types::Any;
use substreams::errors::Error;

#[substreams::handlers::map]
pub fn map_transactions(block: Block) -> Result<TransactionList, Error> {
    // Mutable list to add the output of the Substreams
    let mut transactions: Vec<Transaction> = Vec::new();

    if block.txs.len() != block.tx_results.len() {
        return Err(anyhow!("Transaction list and result list do not match"));
    }

    for i in 0..block.txs.len() {
        let tx_as_bytes = block.txs.get(i).unwrap();
        let tx_as_u8 = &tx_as_bytes[..];

        let tx_result = block.tx_results.get(i).unwrap();
        substreams::log::println("00-----------------");

        if let Ok(tx) = <Tx as prost::Message>::decode(tx_as_u8) {
            if let Some(body) = tx.body {
                let transaction_memo = body.memo;
                let transaction_timeout_height = body.timeout_height;
                let transaction_extension_options = body.extension_options;
                let transaction_non_critical_extension_options = body.non_critical_extension_options;
                substreams::log::println("A-----------------");

                let messages = extract_messages(body.messages);
                substreams::log::println("I-----------------");
                substreams::log::println(messages.len().to_string());

                let transaction = Transaction {
                    raw_tx: tx_as_bytes.to_vec(),
                    memo: transaction_memo,
                    messages: messages,
                    timeout_height: transaction_timeout_height,
                    extension_options: transaction_extension_options,
                    non_critical_extension_options: transaction_non_critical_extension_options,
                    result_code: tx_result.code,
                    result_data: tx_result.data.to_vec(),
                    result_log: tx_result.log.to_string(),
                    result_info: tx_result.info.to_string(),
                    result_gas_wanted: tx_result.gas_wanted,
                    result_gas_used: tx_result.gas_used,
                    result_events: tx_result.events.to_vec(),
                    result_codespace: tx_result.codespace.to_string(),
                    auth_info: tx.auth_info,
                    signatures: tx.signatures,
                };

                transactions.push(transaction);
            }
        }
    }

    Ok(TransactionList { transactions })
}

fn extract_messages(messages: Vec<Any>) -> Vec<Message> {
    return messages
        .iter()
        .map(|message| {
            let message_as_u8 = &message.value[..];

            if message.type_url == "/cosmos.authz.v1beta1.MsgExec" {
                if let Ok(msg_exec) = <MsgExec as prost::Message>::decode(message_as_u8) {
                    return build_message(Value::MsgExec(msg_exec));
                }
            }
            if message.type_url == "/cosmos.bank.v1beta1.MsgSend" {
                if let Ok(msg_send) = <MsgSend as prost::Message>::decode(message_as_u8) {
                    return build_message(Value::MsgSend(msg_send));
                }
            }
            if message.type_url == "/cosmos.bank.v1beta1.MsgMultiSend" {
                if let Ok(msg_multi_send) = <MsgMultiSend as prost::Message>::decode(message_as_u8) {
                    return build_message(Value::MsgMultiSend(msg_multi_send));
                }
            }
            if message.type_url == "/cosmos.crisis.v1beta1.MsgVerifyInvariant" {
                if let Ok(msg_verify_invariant) = <MsgVerifyInvariant as prost::Message>::decode(message_as_u8) {
                    return build_message(Value::MsgVerifyInvariant(msg_verify_invariant));
                }
            }
            if message.type_url == "/cosmos.distribution.v1beta1.MsgWithdrawDelegatorReward" {
                if let Ok(msg_withdraw_delegator_reward) =
                    <MsgWithdrawDelegatorReward as prost::Message>::decode(message_as_u8)
                {
                    return build_message(Value::MsgWithdrawDelegatorReward(msg_withdraw_delegator_reward));
                }
            }
            if message.type_url == "/cosmos.distribution.v1beta1.MsgWithdrawValidatorCommission" {
                if let Ok(msg_withdraw_validator_commission) =
                    <MsgWithdrawValidatorCommission as prost::Message>::decode(message_as_u8)
                {
                    return build_message(Value::MsgWithdrawValidatorCommission(msg_withdraw_validator_commission));
                }
            }
            if message.type_url == "/cosmos.distribution.v1beta1.MsgSetWithdrawAddress" {
                if let Ok(msg_set_withdraw_address) = <MsgSetWithdrawAddress as prost::Message>::decode(message_as_u8) {
                    return build_message(Value::MsgSetWithdrawAddress(msg_set_withdraw_address));
                }
            }
            if message.type_url == "/cosmos.distribution.v1beta1.MsgFundCommunityPool" {
                if let Ok(msg_fund_community_pool) = <MsgFundCommunityPool as prost::Message>::decode(message_as_u8) {
                    return build_message(Value::MsgFundCommunityPool(msg_fund_community_pool));
                }
            }
            if message.type_url == "/cosmos.evidence.v1beta1.MsgSubmitEvidence" {
                if let Ok(msg_submit_evidence) = <MsgSubmitEvidence as prost::Message>::decode(message_as_u8) {
                    return build_message(Value::MsgSubmitEvidence(msg_submit_evidence));
                }
            }
            if message.type_url == "/cosmos.gov.v1beta1.MsgSubmitProposal" {
                if let Ok(msg_submit_proposal) = <MsgSubmitProposal as prost::Message>::decode(message_as_u8) {
                    return build_message(Value::MsgSubmitProposal(msg_submit_proposal));
                }
            }
            if message.type_url == "/cosmos.gov.v1beta1.MsgVote" {
                if let Ok(msg_vote) = <MsgVote as prost::Message>::decode(message_as_u8) {
                    return build_message(Value::MsgVote(msg_vote));
                }
            }
            if message.type_url == "/cosmos.gov.v1beta1.MsgDeposit" {
                if let Ok(msg_deposit) = <MsgDeposit as prost::Message>::decode(message_as_u8) {
                    return build_message(Value::MsgDeposit(msg_deposit));
                }
            }
            if message.type_url == "/cosmos.slashing.v1beta1.MsgUnjail" {
                if let Ok(msg_unjail) = <MsgUnjail as prost::Message>::decode(message_as_u8) {
                    return build_message(Value::MsgUnjail(msg_unjail));
                }
            }

            return build_message(Value::Other(message.clone()));
        })
        .collect();
}

fn build_message(value: Value) -> Message {
    return Message { value: Some(value) };
}