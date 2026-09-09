use crate::pb::protocol;
use crate::pb::sf::tron::r#type::v1::ResponseCode;
use bs58;
use buffa::Enumeration;
use buffa::Message;
use buffa_types::google::protobuf::Any;
use protocol::transaction::contract::ContractType;
use sha2::{Digest, Sha256};

pub fn transaction_failed(status: i32) -> bool {
    status != ResponseCode::Success as i32
}

/// Macro to extract parameter from a contract. Used to extract the from and to addresses from a contract.
macro_rules! extract_param {
    ($struct_type:ty, $parameter:expr, $field:ident) => {
        <$struct_type>::decode_from_slice(&$parameter.value[..])
            .ok()
            .map(|c| c.$field)
    };
}

/// Extracts the 'from' (owner) address from a contract parameter, if available.
/// Returns None if the contract type does not have an owner address.
pub fn extract_from_address(contract_type: i32, parameter: &Any) -> Option<Vec<u8>> {
    match ContractType::from_i32(contract_type) {
        Some(ContractType::TransferContract) => {
            extract_param!(protocol::TransferContract, parameter, owner_address)
        }
        Some(ContractType::TransferAssetContract) => {
            extract_param!(protocol::TransferAssetContract, parameter, owner_address)
        }
        Some(ContractType::VoteAssetContract) => {
            extract_param!(protocol::VoteAssetContract, parameter, owner_address)
        }
        Some(ContractType::VoteWitnessContract) => {
            extract_param!(protocol::VoteWitnessContract, parameter, owner_address)
        }
        Some(ContractType::WitnessCreateContract) => {
            extract_param!(protocol::WitnessCreateContract, parameter, owner_address)
        }
        Some(ContractType::AssetIssueContract) => {
            extract_param!(protocol::AssetIssueContract, parameter, owner_address)
        }
        Some(ContractType::WitnessUpdateContract) => {
            extract_param!(protocol::WitnessUpdateContract, parameter, owner_address)
        }
        Some(ContractType::ParticipateAssetIssueContract) => extract_param!(
            protocol::ParticipateAssetIssueContract,
            parameter,
            owner_address
        ),
        Some(ContractType::AccountUpdateContract) => {
            extract_param!(protocol::AccountUpdateContract, parameter, owner_address)
        }
        Some(ContractType::FreezeBalanceContract) => {
            extract_param!(protocol::FreezeBalanceContract, parameter, owner_address)
        }
        Some(ContractType::UnfreezeBalanceContract) => {
            extract_param!(protocol::UnfreezeBalanceContract, parameter, owner_address)
        }
        Some(ContractType::WithdrawBalanceContract) => {
            extract_param!(protocol::WithdrawBalanceContract, parameter, owner_address)
        }
        Some(ContractType::UnfreezeAssetContract) => {
            extract_param!(protocol::UnfreezeAssetContract, parameter, owner_address)
        }
        Some(ContractType::UpdateAssetContract) => {
            extract_param!(protocol::UpdateAssetContract, parameter, owner_address)
        }
        Some(ContractType::ProposalCreateContract) => {
            extract_param!(protocol::ProposalCreateContract, parameter, owner_address)
        }
        Some(ContractType::ProposalApproveContract) => {
            extract_param!(protocol::ProposalApproveContract, parameter, owner_address)
        }
        Some(ContractType::ProposalDeleteContract) => {
            extract_param!(protocol::ProposalDeleteContract, parameter, owner_address)
        }
        Some(ContractType::SetAccountIdContract) => {
            extract_param!(protocol::SetAccountIdContract, parameter, owner_address)
        }
        Some(ContractType::CreateSmartContract) => {
            extract_param!(protocol::CreateSmartContract, parameter, owner_address)
        }
        Some(ContractType::TriggerSmartContract) => {
            extract_param!(protocol::TriggerSmartContract, parameter, owner_address)
        }
        Some(ContractType::UpdateSettingContract) => {
            extract_param!(protocol::UpdateSettingContract, parameter, owner_address)
        }
        Some(ContractType::ExchangeCreateContract) => {
            extract_param!(protocol::ExchangeCreateContract, parameter, owner_address)
        }
        Some(ContractType::ExchangeInjectContract) => {
            extract_param!(protocol::ExchangeInjectContract, parameter, owner_address)
        }
        Some(ContractType::ExchangeWithdrawContract) => {
            extract_param!(protocol::ExchangeWithdrawContract, parameter, owner_address)
        }
        Some(ContractType::ExchangeTransactionContract) => extract_param!(
            protocol::ExchangeTransactionContract,
            parameter,
            owner_address
        ),
        Some(ContractType::UpdateEnergyLimitContract) => extract_param!(
            protocol::UpdateEnergyLimitContract,
            parameter,
            owner_address
        ),
        Some(ContractType::AccountPermissionUpdateContract) => extract_param!(
            protocol::AccountPermissionUpdateContract,
            parameter,
            owner_address
        ),
        Some(ContractType::ClearAbiContract) => {
            extract_param!(protocol::ClearABIContract, parameter, owner_address)
        }
        Some(ContractType::UpdateBrokerageContract) => {
            extract_param!(protocol::UpdateBrokerageContract, parameter, owner_address)
        }
        // For ShieldedTransferContract, the owner address is in the transparent_from_address field
        Some(ContractType::ShieldedTransferContract) => extract_param!(
            protocol::ShieldedTransferContract,
            parameter,
            transparent_from_address
        ),
        Some(ContractType::MarketSellAssetContract) => {
            extract_param!(protocol::MarketSellAssetContract, parameter, owner_address)
        }
        Some(ContractType::MarketCancelOrderContract) => extract_param!(
            protocol::MarketCancelOrderContract,
            parameter,
            owner_address
        ),
        Some(ContractType::FreezeBalanceV2Contract) => {
            extract_param!(protocol::FreezeBalanceV2Contract, parameter, owner_address)
        }
        Some(ContractType::UnfreezeBalanceV2Contract) => extract_param!(
            protocol::UnfreezeBalanceV2Contract,
            parameter,
            owner_address
        ),
        Some(ContractType::WithdrawExpireUnfreezeContract) => extract_param!(
            protocol::WithdrawExpireUnfreezeContract,
            parameter,
            owner_address
        ),
        Some(ContractType::DelegateResourceContract) => {
            extract_param!(protocol::DelegateResourceContract, parameter, owner_address)
        }
        Some(ContractType::UnDelegateResourceContract) => extract_param!(
            protocol::UnDelegateResourceContract,
            parameter,
            owner_address
        ),
        Some(ContractType::CancelAllUnfreezeV2Contract) => extract_param!(
            protocol::CancelAllUnfreezeV2Contract,
            parameter,
            owner_address
        ),
        Some(ContractType::AccountCreateContract) => {
            extract_param!(protocol::AccountCreateContract, parameter, owner_address)
        }
        // TODO: Validate that there are no owner addresses for these contract types
        Some(ContractType::CustomContract) => None,
        Some(ContractType::GetContract) => None,
        None => None,
    }
}

/// Extracts the 'to' (recipient) address from a contract parameter, if available.
/// Returns None if the contract type does not have a to address.
pub fn extract_to_address(contract_type: i32, parameter: &Any) -> Option<Vec<u8>> {
    match ContractType::from_i32(contract_type)? {
        ContractType::TransferContract => {
            extract_param!(protocol::TransferContract, parameter, to_address)
        }
        ContractType::TransferAssetContract => {
            extract_param!(protocol::TransferAssetContract, parameter, to_address)
        }
        ContractType::ParticipateAssetIssueContract => extract_param!(
            protocol::ParticipateAssetIssueContract,
            parameter,
            to_address
        ),
        // TODO: Do we want to extract all the vote addresses as `to` addresses?
        // https://buf.build/streamingfast/tron-protocol/file/main:core/vote_asset_contract.proto
        // ContractType::VoteAssetContract => {
        //     extract_repeated!(protocol::VoteAssetContract, parameter, vote_address)
        // }
        // TODO: Is this the correct to address?
        // https://buf.build/streamingfast/tron-protocol/docs/main:protocol#protocol.UnDelegateResourceContract
        ContractType::UnDelegateResourceContract => extract_param!(
            protocol::UnDelegateResourceContract,
            parameter,
            receiver_address
        ),
        // TODO: Is this the correct to address?
        // https://buf.build/streamingfast/tron-protocol/docs/main:protocol#protocol.DelegateResourceContract
        ContractType::DelegateResourceContract => extract_param!(
            protocol::DelegateResourceContract,
            parameter,
            receiver_address
        ),
        // TODO: Is this the correct to address?
        // https://buf.build/streamingfast/tron-protocol/docs/main:protocol#protocol.UnfreezeBalanceContract
        ContractType::UnfreezeBalanceContract => extract_param!(
            protocol::UnfreezeBalanceContract,
            parameter,
            receiver_address
        ),
        // TODO: Is this the correct to address?
        // https://buf.build/streamingfast/tron-protocol/docs/main:protocol#protocol.FreezeBalanceContract
        ContractType::FreezeBalanceContract => {
            extract_param!(protocol::FreezeBalanceContract, parameter, receiver_address)
        }
        // TODO: Is this the correct to address?
        // https://buf.build/streamingfast/tron-protocol/docs/main:protocol#protocol.ShieldedTransferContract
        ContractType::ShieldedTransferContract => extract_param!(
            protocol::ShieldedTransferContract,
            parameter,
            transparent_to_address
        ),
        // TODO: Is this the correct to address?
        // https://buf.build/streamingfast/tron-protocol/docs/main:protocol#protocol.AccountCreateContract
        ContractType::AccountCreateContract => {
            extract_param!(protocol::AccountCreateContract, parameter, account_address)
        }
        // TODO: Validate this, there are no to addresses for these other contract types
        _ => None,
    }
}

/// Converts Tron address bytes to a Base58Check-encoded string (with checksum).
/// The address bytes must already include the Tron prefix (0x41).
pub fn tron_address_to_base58(address: &[u8]) -> String {
    let hash1 = Sha256::digest(address);
    let hash2 = Sha256::digest(&hash1);
    let checksum = &hash2[0..4];
    let mut payload = address.to_vec();
    payload.extend_from_slice(checksum);
    bs58::encode(payload).into_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pb::protocol;
    use crate::pb::protocol::transaction::contract::ContractType;
    use base64::Engine;
    use buffa_types::google::protobuf::Any;

    // Helper macro to automate test generation for contract types with owner_address
    macro_rules! test_owner_extract {
        ($name:ident, $contract_type:expr, $struct_type:ty, $field:ident) => {
            #[test]
            fn $name() {
                let owner = vec![1, 2, 3, 4];
                let mut contract = <$struct_type>::default();
                contract.$field = owner.clone();
                let any = Any::pack(
                    &contract,
                    format!("type.googleapis.com/{}", stringify!($struct_type)),
                );
                let result = extract_from_address($contract_type as i32, &any);
                assert_eq!(result, Some(owner));
            }
        };
    }

    // Helper macro to automate test generation for contract types with to_address
    macro_rules! test_to_extract {
        ($name:ident, $contract_type:expr, $struct_type:ty, $field:ident) => {
            #[test]
            fn $name() {
                let to = vec![9, 8, 7, 6];
                let mut contract = <$struct_type>::default();
                contract.$field = to.clone();
                let any = Any::pack(
                    &contract,
                    format!("type.googleapis.com/{}", stringify!($struct_type)),
                );
                let result = extract_to_address($contract_type as i32, &any);
                assert_eq!(result, Some(to));
            }
        };
    }

    test_owner_extract!(
        extract_transfer_contract,
        ContractType::TransferContract,
        protocol::TransferContract,
        owner_address
    );
    test_owner_extract!(
        extract_transfer_asset_contract,
        ContractType::TransferAssetContract,
        protocol::TransferAssetContract,
        owner_address
    );
    test_owner_extract!(
        extract_vote_asset_contract,
        ContractType::VoteAssetContract,
        protocol::VoteAssetContract,
        owner_address
    );
    test_owner_extract!(
        extract_vote_witness_contract,
        ContractType::VoteWitnessContract,
        protocol::VoteWitnessContract,
        owner_address
    );
    test_owner_extract!(
        extract_witness_create_contract,
        ContractType::WitnessCreateContract,
        protocol::WitnessCreateContract,
        owner_address
    );
    test_owner_extract!(
        extract_asset_issue_contract,
        ContractType::AssetIssueContract,
        protocol::AssetIssueContract,
        owner_address
    );
    test_owner_extract!(
        extract_witness_update_contract,
        ContractType::WitnessUpdateContract,
        protocol::WitnessUpdateContract,
        owner_address
    );
    test_owner_extract!(
        extract_participate_asset_issue_contract,
        ContractType::ParticipateAssetIssueContract,
        protocol::ParticipateAssetIssueContract,
        owner_address
    );
    test_owner_extract!(
        extract_account_update_contract,
        ContractType::AccountUpdateContract,
        protocol::AccountUpdateContract,
        owner_address
    );
    test_owner_extract!(
        extract_freeze_balance_contract,
        ContractType::FreezeBalanceContract,
        protocol::FreezeBalanceContract,
        owner_address
    );
    test_owner_extract!(
        extract_unfreeze_balance_contract,
        ContractType::UnfreezeBalanceContract,
        protocol::UnfreezeBalanceContract,
        owner_address
    );
    test_owner_extract!(
        extract_withdraw_balance_contract,
        ContractType::WithdrawBalanceContract,
        protocol::WithdrawBalanceContract,
        owner_address
    );
    test_owner_extract!(
        extract_unfreeze_asset_contract,
        ContractType::UnfreezeAssetContract,
        protocol::UnfreezeAssetContract,
        owner_address
    );
    test_owner_extract!(
        extract_update_asset_contract,
        ContractType::UpdateAssetContract,
        protocol::UpdateAssetContract,
        owner_address
    );
    test_owner_extract!(
        extract_proposal_create_contract,
        ContractType::ProposalCreateContract,
        protocol::ProposalCreateContract,
        owner_address
    );
    test_owner_extract!(
        extract_proposal_approve_contract,
        ContractType::ProposalApproveContract,
        protocol::ProposalApproveContract,
        owner_address
    );
    test_owner_extract!(
        extract_proposal_delete_contract,
        ContractType::ProposalDeleteContract,
        protocol::ProposalDeleteContract,
        owner_address
    );
    test_owner_extract!(
        extract_set_account_id_contract,
        ContractType::SetAccountIdContract,
        protocol::SetAccountIdContract,
        owner_address
    );
    test_owner_extract!(
        extract_create_smart_contract,
        ContractType::CreateSmartContract,
        protocol::CreateSmartContract,
        owner_address
    );
    test_owner_extract!(
        extract_trigger_smart_contract,
        ContractType::TriggerSmartContract,
        protocol::TriggerSmartContract,
        owner_address
    );
    test_owner_extract!(
        extract_update_setting_contract,
        ContractType::UpdateSettingContract,
        protocol::UpdateSettingContract,
        owner_address
    );
    test_owner_extract!(
        extract_exchange_create_contract,
        ContractType::ExchangeCreateContract,
        protocol::ExchangeCreateContract,
        owner_address
    );
    test_owner_extract!(
        extract_exchange_inject_contract,
        ContractType::ExchangeInjectContract,
        protocol::ExchangeInjectContract,
        owner_address
    );
    test_owner_extract!(
        extract_exchange_withdraw_contract,
        ContractType::ExchangeWithdrawContract,
        protocol::ExchangeWithdrawContract,
        owner_address
    );
    test_owner_extract!(
        extract_exchange_transaction_contract,
        ContractType::ExchangeTransactionContract,
        protocol::ExchangeTransactionContract,
        owner_address
    );
    test_owner_extract!(
        extract_update_energy_limit_contract,
        ContractType::UpdateEnergyLimitContract,
        protocol::UpdateEnergyLimitContract,
        owner_address
    );
    test_owner_extract!(
        extract_account_permission_update_contract,
        ContractType::AccountPermissionUpdateContract,
        protocol::AccountPermissionUpdateContract,
        owner_address
    );
    test_owner_extract!(
        extract_clear_abi_contract,
        ContractType::ClearAbiContract,
        protocol::ClearABIContract,
        owner_address
    );
    test_owner_extract!(
        extract_update_brokerage_contract,
        ContractType::UpdateBrokerageContract,
        protocol::UpdateBrokerageContract,
        owner_address
    );
    test_owner_extract!(
        extract_market_sell_asset_contract,
        ContractType::MarketSellAssetContract,
        protocol::MarketSellAssetContract,
        owner_address
    );
    test_owner_extract!(
        extract_market_cancel_order_contract,
        ContractType::MarketCancelOrderContract,
        protocol::MarketCancelOrderContract,
        owner_address
    );
    test_owner_extract!(
        extract_freeze_balance_v2_contract,
        ContractType::FreezeBalanceV2Contract,
        protocol::FreezeBalanceV2Contract,
        owner_address
    );
    test_owner_extract!(
        extract_unfreeze_balance_v2_contract,
        ContractType::UnfreezeBalanceV2Contract,
        protocol::UnfreezeBalanceV2Contract,
        owner_address
    );
    test_owner_extract!(
        extract_withdraw_expire_unfreeze_contract,
        ContractType::WithdrawExpireUnfreezeContract,
        protocol::WithdrawExpireUnfreezeContract,
        owner_address
    );
    test_owner_extract!(
        extract_delegate_resource_contract,
        ContractType::DelegateResourceContract,
        protocol::DelegateResourceContract,
        owner_address
    );
    test_owner_extract!(
        extract_undelegate_resource_contract,
        ContractType::UnDelegateResourceContract,
        protocol::UnDelegateResourceContract,
        owner_address
    );
    test_owner_extract!(
        extract_cancel_all_unfreeze_v2_contract,
        ContractType::CancelAllUnfreezeV2Contract,
        protocol::CancelAllUnfreezeV2Contract,
        owner_address
    );
    test_owner_extract!(
        extract_account_create_contract,
        ContractType::AccountCreateContract,
        protocol::AccountCreateContract,
        owner_address
    );

    #[test]
    fn extract_shielded_transfer_contract() {
        let owner = vec![7, 7, 7, 7];
        let mut contract = protocol::ShieldedTransferContract::default();
        contract.transparent_from_address = owner.clone();
        let any = Any::pack(
            &contract,
            "type.googleapis.com/protocol.ShieldedTransferContract",
        );
        let result = extract_from_address(ContractType::ShieldedTransferContract as i32, &any);
        assert_eq!(result, Some(owner));
    }

    #[test]
    fn extract_custom_contract_none() {
        let any = Any {
            type_url: "type.googleapis.com/protocol.CustomContract".to_string(),
            ..Default::default()
        };
        let result = extract_from_address(ContractType::CustomContract as i32, &any);
        assert_eq!(result, None);
    }

    #[test]
    fn extract_get_contract_none() {
        let any = Any {
            type_url: "type.googleapis.com/protocol.GetContract".to_string(),
            ..Default::default()
        };
        let result = extract_from_address(ContractType::GetContract as i32, &any);
        assert_eq!(result, None);
    }

    #[test]
    fn test_tron_address_to_base58() {
        // Example: QVeYR7tyCAZ4qm6OjPuRnxjT105C (base64) => THxNDMy3y9NP7Bfat9CTKmZ4PFfj1v4gWa
        let base64 = "QVeYR7tyCAZ4qm6OjPuRnxjT105C";
        let expected = "THxNDMy3y9NP7Bfat9CTKmZ4PFfj1v4gWa";
        let bytes = base64::engine::general_purpose::STANDARD
            .decode(base64)
            .unwrap();
        let addr = tron_address_to_base58(&bytes);
        assert_eq!(addr, expected);
    }

    test_to_extract!(
        extract_to_transfer_contract,
        ContractType::TransferContract,
        protocol::TransferContract,
        to_address
    );
    test_to_extract!(
        extract_to_transfer_asset_contract,
        ContractType::TransferAssetContract,
        protocol::TransferAssetContract,
        to_address
    );
    test_to_extract!(
        extract_to_participate_asset_issue_contract,
        ContractType::ParticipateAssetIssueContract,
        protocol::ParticipateAssetIssueContract,
        to_address
    );
    test_to_extract!(
        extract_to_undelegate_resource_contract,
        ContractType::UnDelegateResourceContract,
        protocol::UnDelegateResourceContract,
        receiver_address
    );
    test_to_extract!(
        extract_to_delegate_resource_contract,
        ContractType::DelegateResourceContract,
        protocol::DelegateResourceContract,
        receiver_address
    );
    test_to_extract!(
        extract_to_unfreeze_balance_contract,
        ContractType::UnfreezeBalanceContract,
        protocol::UnfreezeBalanceContract,
        receiver_address
    );
    test_to_extract!(
        extract_to_freeze_balance_contract,
        ContractType::FreezeBalanceContract,
        protocol::FreezeBalanceContract,
        receiver_address
    );
    test_to_extract!(
        extract_to_shielded_transfer_contract,
        ContractType::ShieldedTransferContract,
        protocol::ShieldedTransferContract,
        transparent_to_address
    );
    test_to_extract!(
        extract_to_account_create_contract,
        ContractType::AccountCreateContract,
        protocol::AccountCreateContract,
        account_address
    );
}
