mod pb;

use core::panic;
use std::collections::HashMap;

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
use anyhow::anyhow;
use pb::cosmwasm::wasm::v1::MsgExecuteContract;
use pb::ibc::core::channel::v1::MsgAcknowledgement;
use pb::ibc::core::client::v1::MsgUpdateClient;
use pb::injective::auction::v1beta1::MsgBid;
use pb::injective::exchange::v1beta1::MsgBatchUpdateOrders;
use pb::injective::exchange::v1beta1::MsgDeposit as InjMsgDeposit;
use pb::injective::oracle::v1beta1::MsgRelayProviderPrices;
use pb::injective::peggy::v1::MsgRequestBatch;
use pb::injective::wasmx::v1::MsgExecuteContractCompat;
use pb::injective::wasmx::v1::MsgRegisterContract;
use pb::sf::substreams::cosmos::v1::transaction::message::Value;
use pb::sf::substreams::cosmos::v1::transaction::Message;
use pb::sf::substreams::cosmos::v1::*;
use pb::sf::substreams::v1::Clock;
use prost_types::Any;
use sha2::{Digest, Sha256};
use substreams::errors::Error;
use substreams::log;
use substreams::matches_keys_in_parsed_expr;
use substreams::pb::sf::substreams::index::v1::Keys;

#[substreams::handlers::map]
pub fn all_transactions(block: Block) -> Result<TransactionList, Error> {
    // Mutable list to add the output of the Substreams
    let mut transactions: Vec<Transaction> = Vec::new();

    if block.txs.len() != block.tx_results.len() {
        return Err(anyhow!("Transaction list and result list do not match"));
    }

    for i in 0..block.txs.len() {
        let tx_as_bytes = block.txs.get(i).unwrap();
        let tx_as_u8 = &tx_as_bytes[..];

        let tx_result = block.tx_results.get(i).unwrap();
        // substreams::log::println("00-----------------");

        if let Ok(tx) = <Tx as prost::Message>::decode(tx_as_u8) {
            if let Some(body) = tx.body {
                let transaction_memo = body.memo;
                let transaction_timeout_height = body.timeout_height;
                let transaction_extension_options = body.extension_options;
                let transaction_non_critical_extension_options = body.non_critical_extension_options;
                // substreams::log::println("A-----------------");

                let messages = extract_messages(body.messages);
                // substreams::log::println("I-----------------");
                // substreams::log::println(messages.len().to_string());

                let transaction = Transaction {
                    raw_tx: tx_as_bytes.to_vec(),
                    hash: compute_tx_hash(tx_as_bytes),
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

    Ok(TransactionList {
        transactions: transactions,
        clock: Some(Clock {
            id: hex::encode(block.hash),
            number: block.height as u64,
            timestamp: block.time,
        }),
    })
}

#[substreams::handlers::map]
pub fn all_events(block: Block) -> Result<EventList, Error> {
    // Mutable list to add the output of the Substreams
    let mut events: Vec<Event> = Vec::new();

    if block.txs.len() != block.tx_results.len() {
        return Err(anyhow!("Transaction list and result list do not match"));
    }

    // block events are the combination of BeginBlockEvents and EndBlockEvents 
    events.extend(block.events.into_iter().map(|event| {
        return Event {
            event: Some(event),
            transaction_hash: "".to_string(),
        };
    }));

    for (i, tx_result) in block.tx_results.into_iter().enumerate() {
        let tx_hash = compute_tx_hash(block.txs.get(i).unwrap());

        let block_events: Vec<Event> = tx_result
            .events
            .into_iter()
            .map(|event| {
                return Event {
                    event: Some(event),
                    transaction_hash: tx_hash.clone(),
                };
            })
            .collect();

        events.extend(block_events);
    }

    Ok(EventList {
        events: events,
        clock: Some(Clock {
            id: hex::encode(block.hash),
            number: block.height as u64,
            timestamp: block.time,
        }),
    })
}

#[substreams::handlers::map]
fn index_events(events: EventList) -> Result<Keys, Error> {
    let mut keys = Keys::default();

    events.events.into_iter().for_each(|e| {
        if let Some(ev) = e.event {
            keys.keys.push(format!("type:{}", ev.r#type));
            ev.attributes.into_iter().for_each(|attr| {
                keys.keys.push(format!("attr:{}", attr.key));
            });
        }
    });

    Ok(keys)
}

#[substreams::handlers::map]
fn filtered_events(query: String, events: EventList) -> Result<EventList, Error> {
    let filtered: Vec<Event> = events
        .events
        .into_iter()
        .filter(|e| {
            if let Some(ev) = &e.event {
                let mut keys = Vec::new();
                keys.push(format!("type:{}", ev.r#type.clone()));
                ev.attributes.iter().for_each(|attr| {
                    keys.push(format!("attr:{}", attr.key));
                });
                matches_keys_in_parsed_expr(&keys, &query).expect("matching events from query")
            } else {
                false
            }
        })
        .collect();

    if filtered.len() == 0 {
        return Ok(EventList::default());
    }
    Ok(EventList {
        events: filtered,
        clock: events.clock,
    })
}

#[substreams::handlers::map]
fn filtered_event_groups(query: String, events: EventList) -> Result<EventList, Error> {
    let matching_trx_hashes= events
        .events
        .iter()
        .filter(|e| {
            if let Some(ev) = &e.event {
                let mut keys = Vec::new();
                keys.push(format!("type:{}", ev.r#type.clone()));
                ev.attributes.iter().for_each(|attr| {
                    keys.push(format!("attr:{}", attr.key));
                });
                matches_keys_in_parsed_expr(&keys, &query).expect("matching events from query")
            } else {
                false
            }
        })
        .map(|e| (e.transaction_hash.to_string(), true))
        .collect::<HashMap<String, bool>>();

    let filtered: Vec<Event> = events
        .events
        .into_iter()
        .filter(|e| {
            matching_trx_hashes.contains_key(e.transaction_hash.as_str())
        })
        .collect();

    if filtered.len() == 0 {
        return Ok(EventList::default());
    }
    Ok(EventList {
        events: filtered,
        clock: events.clock,
    })
}

#[substreams::handlers::map]
fn filtered_trx_by_events(query: String, trxs: TransactionList) -> Result<TransactionList, Error> {
    let events = trxs.transactions.iter().map(|t| {
        t.result_events.iter().map(|e| {
            Event {
                transaction_hash: t.hash.clone(),
                event: Some(e.clone()),
            }
        })
    }).flatten().collect::<Vec<_>>();

    let matching_trx_hashes= events
        .iter()
        .filter(|e| {
            if let Some(ev) = &e.event {
                let mut keys = Vec::new();
                keys.push(format!("type:{}", ev.r#type.clone()));
                ev.attributes.iter().for_each(|attr| {
                    keys.push(format!("attr:{}", attr.key));
                });
                matches_keys_in_parsed_expr(&keys, &query).expect("matching events from query")
            } else {
                false
            }
        })
        .map(|e| (e.transaction_hash.to_string(), true))
        .collect::<HashMap<String, bool>>();

    let transactions: Vec<Transaction> = trxs
        .transactions
        .into_iter()
        .filter(|t| matching_trx_hashes.contains_key(t.hash.as_str()))
        .collect();

    if transactions.len() == 0 {
        return Ok(TransactionList::default());
    }
    Ok(TransactionList {
        transactions: transactions,
        clock: trxs.clock,
    })
}

#[substreams::handlers::map]
fn filtered_events_by_attribute_value(query: String, events: EventList) -> Result<EventList, Error> {
    let filtered: Vec<Event> = events
        .events
        .into_iter()
        .filter(|e| {
            if let Some(ev) = &e.event {
                let mut keys = Vec::new();
                keys.push(format!("type:{}", ev.r#type.clone()));
                ev.attributes.iter().for_each(|attr| {
                    keys.push(format!("attr:{}", attr.key));
                    keys.push(format!("attr:{}:{}", attr.key, attr.value));
                });
                matches_keys_in_parsed_expr(&keys, &query).expect("matching events from query")
            } else {
                false
            }
        })
        .collect();

    if filtered.len() == 0 {
        return Ok(EventList::default());
    }
    Ok(EventList {
        events: filtered,
        clock: events.clock,
    })
}

#[substreams::handlers::map]
fn filtered_event_groups_by_attribute_value(query: String, events: EventList) -> Result<EventList, Error> {
    let matching_trx_hashes= events
        .events
        .iter()
        .filter(|e| {
            if let Some(ev) = &e.event {
                let mut keys = Vec::new();
                keys.push(format!("type:{}", ev.r#type.clone()));
                ev.attributes.iter().for_each(|attr| {
                    keys.push(format!("attr:{}", attr.key));
                    keys.push(format!("attr:{}:{}", attr.key, attr.value));
                });
                matches_keys_in_parsed_expr(&keys, &query).expect("matching events from query")
            } else {
                false
            }
        })
        .map(|e| (e.transaction_hash.to_string(), true))
        .collect::<HashMap<String, bool>>();

    let filtered: Vec<Event> = events
        .events
        .into_iter()
        .filter(|e| {
            matching_trx_hashes.contains_key(e.transaction_hash.as_str())
        })
        .collect();

    if filtered.len() == 0 {
        return Ok(EventList::default());
    }
    Ok(EventList {
        events: filtered,
        clock: events.clock,
    })
}

#[substreams::handlers::map]
fn filtered_trx_by_events_attribute_value(query: String, events: EventList, trxs: TransactionList) -> Result<TransactionList, Error> {
    let matching_trx_hashes= events
        .events
        .iter()
        .filter(|e| {
            if let Some(ev) = &e.event {
                let mut keys = Vec::new();
                keys.push(format!("type:{}", ev.r#type.clone()));
                ev.attributes.iter().for_each(|attr| {
                    keys.push(format!("attr:{}", attr.key));
                    keys.push(format!("attr:{}:{}", attr.key, attr.value));
                });
                matches_keys_in_parsed_expr(&keys, &query).expect("matching events from query")
            } else {
                false
            }
        })
        .map(|e| (e.transaction_hash.to_string(), true))
        .collect::<HashMap<String, bool>>();

    let transactions: Vec<Transaction> = trxs
        .transactions
        .into_iter()
        .filter(|t| matching_trx_hashes.contains_key(t.hash.as_str()))
        .collect();

    if transactions.len() == 0 {
        return Ok(TransactionList::default());
    }
    Ok(TransactionList {
        transactions: transactions,
        clock: trxs.clock,
    })
}

fn extract_messages(messages: Vec<Any>) -> Vec<Message> {
    return messages
        .iter()
        .enumerate()
        .map(|(u, message)| {
            let message_as_u8 = &message.value[..];
            let i = u.try_into().unwrap();

            match message.type_url.as_str() {
                "/cosmos.authz.v1beta1.MsgExec" => {
                    if let Ok(msg_exec) = <MsgExec as prost::Message>::decode(message_as_u8) {
                        return build_message(Value::MsgExec(msg_exec), i);
                    }
                }
                "/cosmos.bank.v1beta1.MsgSend" => {
                    if let Ok(msg_send) = <MsgSend as prost::Message>::decode(message_as_u8) {
                        return build_message(Value::MsgSend(msg_send), i);
                    }
                }
                "/cosmos.bank.v1beta1.MsgMultiSend" => {
                    if let Ok(msg_multi_send) = <MsgMultiSend as prost::Message>::decode(message_as_u8) {
                        return build_message(Value::MsgMultiSend(msg_multi_send), i);
                    }
                }
                "/cosmos.crisis.v1beta1.MsgVerifyInvariant" => {
                    if let Ok(msg_verify_invariant) = <MsgVerifyInvariant as prost::Message>::decode(message_as_u8) {
                        return build_message(Value::MsgVerifyInvariant(msg_verify_invariant), i);
                    }
                }
                "/cosmos.distribution.v1beta1.MsgWithdrawDelegatorReward" => {
                    if let Ok(msg_withdraw_delegator_reward) =
                        <MsgWithdrawDelegatorReward as prost::Message>::decode(message_as_u8)
                    {
                        return build_message(Value::MsgWithdrawDelegatorReward(msg_withdraw_delegator_reward), i);
                    }
                }
                "/cosmos.distribution.v1beta1.MsgWithdrawValidatorCommission" => {
                    if let Ok(msg_withdraw_validator_commission) =
                        <MsgWithdrawValidatorCommission as prost::Message>::decode(message_as_u8)
                    {
                        return build_message(
                            Value::MsgWithdrawValidatorCommission(msg_withdraw_validator_commission),
                            i,
                        );
                    }
                }
                "/cosmos.distribution.v1beta1.MsgSetWithdrawAddress" => {
                    if let Ok(msg_set_withdraw_address) =
                        <MsgSetWithdrawAddress as prost::Message>::decode(message_as_u8)
                    {
                        return build_message(Value::MsgSetWithdrawAddress(msg_set_withdraw_address), i);
                    }
                }
                "/cosmos.distribution.v1beta1.MsgFundCommunityPool" => {
                    if let Ok(msg_fund_community_pool) = <MsgFundCommunityPool as prost::Message>::decode(message_as_u8)
                    {
                        return build_message(Value::MsgFundCommunityPool(msg_fund_community_pool), i);
                    }
                }
                "/cosmos.evidence.v1beta1.MsgSubmitEvidence" => {
                    if let Ok(msg_submit_evidence) = <MsgSubmitEvidence as prost::Message>::decode(message_as_u8) {
                        return build_message(Value::MsgSubmitEvidence(msg_submit_evidence), i);
                    }
                }
                "/cosmos.gov.v1beta1.MsgSubmitProposal" => {
                    if let Ok(msg_submit_proposal) = <MsgSubmitProposal as prost::Message>::decode(message_as_u8) {
                        return build_message(Value::MsgSubmitProposal(msg_submit_proposal), i);
                    }
                }
                "/cosmos.gov.v1beta1.MsgVote" => {
                    if let Ok(msg_vote) = <MsgVote as prost::Message>::decode(message_as_u8) {
                        return build_message(Value::MsgVote(msg_vote), i);
                    }
                }
                "/cosmos.gov.v1beta1.MsgDeposit" => {
                    if let Ok(msg_deposit) = <MsgDeposit as prost::Message>::decode(message_as_u8) {
                        return build_message(Value::MsgDeposit(msg_deposit), i);
                    }
                }
                "/cosmos.slashing.v1beta1.MsgUnjail" => {
                    if let Ok(msg_unjail) = <MsgUnjail as prost::Message>::decode(message_as_u8) {
                        return build_message(Value::MsgUnjail(msg_unjail), i);
                    }
                }
                "/injective.exchange.v1beta1.MsgBatchUpdateOrders" => {
                    if let Ok(msg) = <MsgBatchUpdateOrders as prost::Message>::decode(message_as_u8) {
                        return build_message(Value::MsgBatchUpdateOrders(msg), i);
                    }
                }
                "/injective.wasmx.v1.MsgExecuteContractCompat" => {
                    if let Ok(msg) = <MsgExecuteContractCompat as prost::Message>::decode(message_as_u8) {
                        return build_message(Value::MsgExecuteContractCompat(msg), i);
                    }
                }
                "/injective.auction.v1beta1.MsgBid" => {
                    if let Ok(msg) = <MsgBid as prost::Message>::decode(message_as_u8) {
                        return build_message(Value::MsgBid(msg), i);
                    }
                }
                "/injective.exchange.v1beta.MsgDeposit" => {
                    if let Ok(msg) = <InjMsgDeposit as prost::Message>::decode(message_as_u8) {
                        return build_message(Value::InjMsgDeposit(msg), i);
                    }
                }
                "/injective.peggy.v1.MsgRequestBatch" => {
                    if let Ok(msg) = <MsgRequestBatch as prost::Message>::decode(message_as_u8) {
                        return build_message(Value::MsgRequestBatch(msg), i);
                    }
                }
                "/injective.wasmx.v1.MsgRegisterContract" => {
                    if let Ok(msg) = <MsgRegisterContract as prost::Message>::decode(message_as_u8) {
                        return build_message(Value::MsgRegisterContract(msg), i);
                    }
                }

                "/cosmwasm.wasm.v1.MsgExecuteContract" => {
                    if let Ok(msg) = <MsgExecuteContract as prost::Message>::decode(message_as_u8) {
                        return build_message(Value::MsgExecuteContract(msg), i);
                    }
                }
                "/ibc.core.client.v1.MsgUpdateClient" => {
                    if let Ok(msg) = <MsgUpdateClient as prost::Message>::decode(message_as_u8) {
                        return build_message(Value::MsgUpdateClient(msg), i);
                    }
                }
                "/ibc.core.channel.v1.MsgAcknowledgement" => {
                    if let Ok(msg) = <MsgAcknowledgement as prost::Message>::decode(message_as_u8) {
                        return build_message(Value::MsgAcknowledgement(msg), i);
                    }
                }
                "/injective.oracle.v1beta1.MsgRelayProviderPrices" => {
                    if let Ok(msg) = <MsgRelayProviderPrices as prost::Message>::decode(message_as_u8) {
                        return build_message(Value::MsgRelayProviderPrices(msg), i);
                    }
                }
                _ => {
                    log::println(format!("Unsupported message type: {}", message.type_url.as_str()));
                    return build_message(Value::Other(message.clone()), i);
                }
            }

            panic!("Could not decode message type {}", message.type_url.as_str());
        })
        .collect();
}

fn build_message(value: Value, idx: u32) -> Message {
    return Message {
        index: idx,
        value: Some(value),
    };
}

fn compute_tx_hash(tx_as_bytes: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(tx_as_bytes);
    let tx_hash = hasher.finalize();
    return hex::encode(tx_hash);
}
