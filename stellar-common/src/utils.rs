use core::panic;
use std::io::Cursor;
use stellar_xdr::curr::{
    AccountMergeResult, Asset as StellarAsset, Limited, Limits, Price as StellarPrice, ReadXdr, Transaction, TransactionEnvelope, TransactionMeta, TransactionResult, TransactionResultResult
};

use crate::pb::sf::substreams::stellar::r#type::v1::{
    asset::Asset as EnumAsset, Asset, CreditAlphaNum12, CreditAlphaNum4, Price,
};

pub fn decode_transaction(
    result_xdr: &Vec<u8>,
    envelope_xdr: &Vec<u8>,
) -> Result<Transaction, substreams::errors::Error> {
    let trx_result: TransactionResult = match decode_transaction_result(result_xdr) {
        Ok(result) => result,
        Err(_) => panic!("Could not decode transaction result"),
    };

    match trx_result.result {
        TransactionResultResult::TxSuccess(_) => {}
        _ => return Err(substreams::errors::Error::msg("Transaction failed")),
    }

    let trx = match decode_transaction_envelope(envelope_xdr) {
        Ok(trx) => trx,
        Err(_) => panic!("Could not decode transaction envelope"),
    };

    let trx_v1 = match trx {
        TransactionEnvelope::Tx(v1) => v1,
        _ => panic!("Expected TransactionV1Envelope type"),
    };

    return Ok(trx_v1.tx);
}

pub fn decode_transaction_result(result_xdr: &Vec<u8>) -> Result<TransactionResult, stellar_xdr::curr::Error> {
    let buf = Cursor::new(result_xdr);
    let transaction_result = TransactionResult::read_xdr(&mut Limited::new(buf, Limits::none()));
    transaction_result
}

pub fn decode_transaction_meta(result_meta_xdr: &Vec<u8>) -> Result<TransactionMeta, stellar_xdr::curr::Error> {
    let buf = Cursor::new(result_meta_xdr);
    let transaction_meta = TransactionMeta::read_xdr(&mut Limited::new(buf, Limits::none()));
    transaction_meta
}

pub fn decode_account_merge_result(transaction_result: &TransactionResult) -> Option<i64> {
    match &transaction_result.result {
        stellar_xdr::curr::TransactionResultResult::TxSuccess(operation_results) => {
            for operation_result in operation_results.as_vec() {
                if let stellar_xdr::curr::OperationResult::OpInner(
                    stellar_xdr::curr::OperationResultTr::AccountMerge(account_merge_result),
                ) = operation_result
                {
                    if let AccountMergeResult::Success(value) = account_merge_result {
                        return Some(*value);
                    }
                }
            }
            None
        }
        _ => None,
    }
}

fn decode_transaction_envelope(envelope_xdr: &Vec<u8>) -> Result<TransactionEnvelope, stellar_xdr::curr::Error> {
    let buf = Cursor::new(envelope_xdr);
    let transaction_envelope = TransactionEnvelope::read_xdr(&mut Limited::new(buf, Limits::none()));
    transaction_envelope
}

pub fn transaction_failed(status: i32) -> bool {
    status == 2
}

pub fn create_asset(asset: &StellarAsset) -> Asset {
    let mut out_asset = Asset::default();

    match asset {
        StellarAsset::Native => {
            out_asset.asset = Some(EnumAsset::Native("XLM".to_string()));
        }
        StellarAsset::CreditAlphanum4(credit) => {
            out_asset.asset = Some(EnumAsset::CreditAlphaNum4(CreditAlphaNum4 {
                asset_code: credit.asset_code.to_string(),
                issuer: credit.issuer.0.to_string(),
            }));
        }
        StellarAsset::CreditAlphanum12(credit) => {
            out_asset.asset = Some(EnumAsset::CreditAlphaNum12(CreditAlphaNum12 {
                asset_code: credit.asset_code.to_string(),
                issuer: credit.issuer.0.to_string(),
            }));
        }
    }

    out_asset
}

pub fn create_price(price: &StellarPrice) -> Price {
    Price { n: price.n, d: price.d }
}
