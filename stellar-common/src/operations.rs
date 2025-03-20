use crate::{
    pb::sf::substreams::stellar::r#type::v1::{
        operation::Op, AccountMerge, AllowTrust, ClaimClaimableBalance, Clawback,
        ClawbackClaimableBalance, CreateAccount, CreateClaimableBalance, CreatePassiveSellOffer,
        LiquidityPoolDeposit, LiquidityPoolWithdraw, ManageBuyOffer, ManageSellOffer, Operation,
        Operations, PathPaymentStrictReceive, PathPaymentStrictSend, Payment, SetTrustLineFlags,
        Transactions,
    },
    utils,
};
use core::panic;
use std::collections::HashSet;
use substreams::{pb::sf::substreams::index::v1::Keys, Hex};

#[substreams::handlers::map]
fn map_operations(transactions: Transactions) -> Result<Operations, substreams::errors::Error> {
    let mut operations: Operations = Operations::default();

    transactions.transactions.iter().for_each(|transaction| {
        let hash = Hex(&transaction.hash).to_string();
        let trx =
            match utils::decode_transaction(&transaction.result_xdr, &transaction.envelope_xdr) {
                Ok(trx) => trx,
                Err(_) => return,
            };

        // All operations that emit token transfers have been taken from https://github.com/stellar/go/issues/5580
        trx.operations
            .iter()
            .for_each(|operation| match &operation.body {
                // Create account: https://stellar.expert/explorer/testnet/tx/8191ad23e96b7426fd6692cd0cf402e85067ea2ce40c97c6a6562bfffe71a0b2
                stellar_xdr::curr::OperationBody::CreateAccount(create_account_op) => {
                    operations.operations.push(Operation {
                        op: Some(Op::CreateAccount(CreateAccount {
                            trx_hash: hash.clone(),
                            ledger_sequence: transaction.block_number,
                            destination: create_account_op.destination.to_string(),
                            starting_balance: create_account_op.starting_balance,
                        })),
                    });
                }
                // Merge Account: https://stellar.expert/explorer/testnet/tx/2bab87af23a13fbe40c363dd326767bb85b00692a3428906f080f2a2a80423c6
                stellar_xdr::curr::OperationBody::AccountMerge(muxed_account) => {
                    let result_xdr = match utils::decode_transaction_result(&transaction.result_xdr)
                    {
                        Ok(result) => result,
                        Err(_) => return,
                    };
                    match utils::decode_account_merge_result(&result_xdr) {
                        Some(amount) => {
                            operations.operations.push(Operation {
                                op: Some(Op::AccountMerge(AccountMerge {
                                    trx_hash: hash.clone(),
                                    ledger_sequence: transaction.block_number,
                                    account: muxed_account.to_string(),
                                    amount,
                                })),
                            });
                        }
                        None => return,
                    }
                }
                stellar_xdr::curr::OperationBody::Payment(payment) => {
                    operations.operations.push(Operation {
                        op: Some(Op::Payment(Payment {
                            trx_hash: hash.clone(),
                            ledger_sequence: transaction.block_number,
                            destination: payment.destination.to_string(),
                            asset: Some(utils::create_asset(&payment.asset)),
                            amount: payment.amount,
                        })),
                    });
                }
                stellar_xdr::curr::OperationBody::CreateClaimableBalance(create_claim_balance) => {
                    operations.operations.push(Operation {
                        op: Some(Op::CreateClaimableBalance(CreateClaimableBalance {
                            trx_hash: hash.clone(),
                            ledger_sequence: transaction.block_number,
                            asset: Some(utils::create_asset(&create_claim_balance.asset)),
                            amount: create_claim_balance.amount,
                            claimants: create_claim_balance
                                .claimants
                                .iter()
                                .flat_map(|claimant| match claimant {
                                    stellar_xdr::curr::Claimant::ClaimantTypeV0(claimant_v0) => {
                                        Some(claimant_v0.destination.to_string())
                                    }
                                })
                                .collect(),
                        })),
                    });
                }
                stellar_xdr::curr::OperationBody::ClaimClaimableBalance(claim_claim_balance) => {
                    operations.operations.push(Operation {
                        op: Some(Op::ClaimClaimableBalance(ClaimClaimableBalance {
                            trx_hash: hash.clone(),
                            ledger_sequence: transaction.block_number,
                            balance_id: claim_claim_balance.balance_id.to_string(),
                        })),
                    });
                }
                stellar_xdr::curr::OperationBody::Clawback(clawback) => {
                    operations.operations.push(Operation {
                        op: Some(Op::Clawback(Clawback {
                            trx_hash: hash.clone(),
                            ledger_sequence: transaction.block_number,
                            asset: Some(utils::create_asset(&clawback.asset)),
                            from: clawback.from.to_string(),
                            amount: clawback.amount,
                        })),
                    });
                }
                stellar_xdr::curr::OperationBody::ClawbackClaimableBalance(
                    clawback_claimable_balance,
                ) => {
                    operations.operations.push(Operation {
                        op: Some(Op::ClawbackClaimableBalance(ClawbackClaimableBalance {
                            trx_hash: hash.clone(),
                            ledger_sequence: transaction.block_number,
                            balance_id: clawback_claimable_balance.balance_id.to_string(),
                        })),
                    });
                }
                stellar_xdr::curr::OperationBody::AllowTrust(allow_trust) => {
                    operations.operations.push(Operation {
                        op: Some(Op::AllowTrust(AllowTrust {
                            trx_hash: hash.clone(),
                            ledger_sequence: transaction.block_number,
                            trustor: allow_trust.trustor.to_string(),
                            asset: allow_trust.asset.to_string(),
                            authorize: allow_trust.authorize,
                        })),
                    });
                }
                stellar_xdr::curr::OperationBody::SetTrustLineFlags(set_trust_line_flags) => {
                    operations.operations.push(Operation {
                        op: Some(Op::SetTrustLineFlags(SetTrustLineFlags {
                            trx_hash: hash.clone(),
                            ledger_sequence: transaction.block_number,
                            trustor: set_trust_line_flags.trustor.to_string(),
                            asset: Some(utils::create_asset(&set_trust_line_flags.asset)),
                            clear_flags: set_trust_line_flags.clear_flags,
                            set_flags: set_trust_line_flags.set_flags,
                        })),
                    });
                }
                stellar_xdr::curr::OperationBody::LiquidityPoolDeposit(liquidity_pool_deposit) => {
                    operations.operations.push(Operation {
                        op: Some(Op::LiquidityPoolDeposit(LiquidityPoolDeposit {
                            trx_hash: hash.clone(),
                            ledger_sequence: transaction.block_number,
                            liquidity_pool_id: liquidity_pool_deposit
                                .liquidity_pool_id
                                .0
                                .to_string(),
                            max_amount_a: liquidity_pool_deposit.max_amount_a,
                            max_amount_b: liquidity_pool_deposit.max_amount_b,
                            min_price: Some(utils::create_price(&liquidity_pool_deposit.min_price)),
                            max_price: Some(utils::create_price(&liquidity_pool_deposit.max_price)),
                        })),
                    });
                }
                stellar_xdr::curr::OperationBody::LiquidityPoolWithdraw(
                    liquidity_pool_withdraw,
                ) => {
                    operations.operations.push(Operation {
                        op: Some(Op::LiquidityPoolWithdraw(LiquidityPoolWithdraw {
                            trx_hash: hash.clone(),
                            ledger_sequence: transaction.block_number,
                            liquidity_pool_id: liquidity_pool_withdraw
                                .liquidity_pool_id
                                .0
                                .to_string(),
                            amount: liquidity_pool_withdraw.amount,
                            min_amount_a: liquidity_pool_withdraw.min_amount_a,
                            min_amount_b: liquidity_pool_withdraw.min_amount_b,
                        })),
                    });
                }
                stellar_xdr::curr::OperationBody::ManageBuyOffer(manage_buy_offer) => {
                    operations.operations.push(Operation {
                        op: Some(Op::ManageBuyOffer(ManageBuyOffer {
                            trx_hash: hash.clone(),
                            ledger_sequence: transaction.block_number,
                            selling: Some(utils::create_asset(&manage_buy_offer.selling)),
                            buying: Some(utils::create_asset(&manage_buy_offer.buying)),
                            buy_amount: manage_buy_offer.buy_amount,
                            price: Some(utils::create_price(&manage_buy_offer.price)),
                            offer_id: manage_buy_offer.offer_id,
                        })),
                    });
                }
                stellar_xdr::curr::OperationBody::ManageSellOffer(manage_sell_offer) => {
                    operations.operations.push(Operation {
                        op: Some(Op::ManageSellOffer(ManageSellOffer {
                            trx_hash: hash.clone(),
                            ledger_sequence: transaction.block_number,
                            selling: Some(utils::create_asset(&manage_sell_offer.selling)),
                            buying: Some(utils::create_asset(&manage_sell_offer.buying)),
                            amount: manage_sell_offer.amount,
                            price: Some(utils::create_price(&manage_sell_offer.price)),
                            offer_id: manage_sell_offer.offer_id,
                        })),
                    });
                }
                stellar_xdr::curr::OperationBody::CreatePassiveSellOffer(create_passive_sell) => {
                    operations.operations.push(Operation {
                        op: Some(Op::CreatePassiveSellOffer(CreatePassiveSellOffer {
                            trx_hash: hash.clone(),
                            ledger_sequence: transaction.block_number,
                            selling: Some(utils::create_asset(&create_passive_sell.selling)),
                            buying: Some(utils::create_asset(&create_passive_sell.buying)),
                            amount: create_passive_sell.amount,
                            price: Some(utils::create_price(&create_passive_sell.price)),
                        })),
                    });
                }
                stellar_xdr::curr::OperationBody::PathPaymentStrictSend(
                    path_payment_strict_send,
                ) => {
                    operations.operations.push(Operation {
                        op: Some(Op::PathPaymentStrictSend(PathPaymentStrictSend {
                            trx_hash: hash.clone(),
                            ledger_sequence: transaction.block_number,
                            send_asset: Some(utils::create_asset(
                                &path_payment_strict_send.send_asset,
                            )),
                            send_amount: path_payment_strict_send.send_amount,
                            destination: path_payment_strict_send.destination.to_string(),
                            dest_asset: Some(utils::create_asset(
                                &path_payment_strict_send.dest_asset,
                            )),
                            dest_min: path_payment_strict_send.dest_min,
                            path: path_payment_strict_send
                                .path
                                .iter()
                                .flat_map(|asset| Some(utils::create_asset(&asset)))
                                .collect(),
                        })),
                    });
                }
                stellar_xdr::curr::OperationBody::PathPaymentStrictReceive(
                    path_payment_strict_receive,
                ) => {
                    operations.operations.push(Operation {
                        op: Some(Op::PathPaymentStrictReceive(PathPaymentStrictReceive {
                            trx_hash: hash.clone(),
                            ledger_sequence: transaction.block_number,
                            send_asset: Some(utils::create_asset(
                                &path_payment_strict_receive.send_asset,
                            )),
                            send_max: path_payment_strict_receive.send_max,
                            destination: path_payment_strict_receive.destination.to_string(),
                            dest_asset: Some(utils::create_asset(
                                &path_payment_strict_receive.dest_asset,
                            )),
                            dest_amount: path_payment_strict_receive.dest_amount,
                            path: path_payment_strict_receive
                                .path
                                .iter()
                                .flat_map(|asset| Some(utils::create_asset(&asset)))
                                .collect(),
                        })),
                    });
                }
                _ => {}
            });
    });

    Ok(operations)
}

#[substreams::handlers::map]
fn index_operations(operations: Operations) -> Result<Keys, substreams::errors::Error> {
    let keys: HashSet<String> = operations
        .operations
        .iter()
        .filter_map(|operation| operation.op.as_ref())
        .map(|operation| format!("operation:{}", name_from_operation(operation)))
        .collect();

    let keys_string: Vec<String> = keys.into_iter().collect();
    Ok(Keys { keys: keys_string })
}

#[substreams::handlers::map]
fn filtered_operations(
    query: String,
    operations: Operations,
) -> Result<Operations, substreams::errors::Error> {
    let query = substreams::expr_matcher(&query);

    let filtered_operations: Vec<Operation> = operations
        .operations
        .into_iter()
        .filter(|operation| {
            if operation.op.is_none() {
                return false;
            }

            query.matches_keys(&vec![format!(
                "operation:{}",
                name_from_operation(&operation.op.as_ref().unwrap())
            )])
        })
        .collect();

    Ok(Operations {
        operations: filtered_operations,
    })
}

fn name_from_operation(operation: &Op) -> &str {
    return match operation {
        &Op::CreateAccount(_) => "create_account",
        &Op::AccountMerge(_) => "account_merge",
        &Op::Payment(_) => "payment",
        &Op::CreateClaimableBalance(_) => "create_claimable_balance",
        &Op::ClaimClaimableBalance(_) => "claim_claimable_balance",
        &Op::Clawback(_) => "clawback",
        &Op::ClawbackClaimableBalance(_) => "clawback_claimable_balance",
        &Op::AllowTrust(_) => "allow_trust",
        &Op::SetTrustLineFlags(_) => "set_trust_line_flags",
        &Op::LiquidityPoolDeposit(_) => "liquidity_pool_deposit",
        &Op::LiquidityPoolWithdraw(_) => "liquidity_pool_withdraw",
        &Op::ManageBuyOffer(_) => "manage_buy_offer",
        &Op::ManageSellOffer(_) => "manage_sell_offer",
        &Op::CreatePassiveSellOffer(_) => "create_passive_sell_offer",
        &Op::PathPaymentStrictSend(_) => "path_payment_strict_send",
        &Op::PathPaymentStrictReceive(_) => "path_payment_strict_receive",
    };
}
