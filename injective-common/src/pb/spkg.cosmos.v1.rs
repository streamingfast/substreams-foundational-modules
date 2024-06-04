// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransactionList {
    #[prost(message, repeated, tag="1")]
    pub transactions: ::prost::alloc::vec::Vec<Transaction>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Transaction {
    /// from the firehose Block as `bytes`
    #[prost(bytes="vec", tag="1")]
    pub raw_tx: ::prost::alloc::vec::Vec<u8>,
    /// from TxBody
    #[prost(string, tag="2")]
    pub memo: ::prost::alloc::string::String,
    #[prost(uint64, tag="3")]
    pub timeout_height: u64,
    #[prost(message, repeated, tag="5")]
    pub messages: ::prost::alloc::vec::Vec<transaction::Message>,
    #[prost(message, repeated, tag="1023")]
    pub extension_options: ::prost::alloc::vec::Vec<::prost_types::Any>,
    #[prost(message, repeated, tag="2047")]
    pub non_critical_extension_options: ::prost::alloc::vec::Vec<::prost_types::Any>,
    /// from TxResult
    #[prost(uint32, tag="21")]
    pub result_code: u32,
    #[prost(bytes="vec", tag="22")]
    pub result_data: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="23")]
    pub result_log: ::prost::alloc::string::String,
    #[prost(string, tag="24")]
    pub result_info: ::prost::alloc::string::String,
    #[prost(int64, tag="25")]
    pub result_gas_wanted: i64,
    #[prost(int64, tag="26")]
    pub result_gas_used: i64,
    #[prost(message, repeated, tag="27")]
    pub result_events: ::prost::alloc::vec::Vec<super::super::super::sf::cosmos::r#type::v2::Event>,
    #[prost(string, tag="28")]
    pub result_codespace: ::prost::alloc::string::String,
    #[prost(message, optional, tag="32")]
    pub auth_info: ::core::option::Option<super::super::super::cosmos::tx::v1beta1::AuthInfo>,
    #[prost(bytes="vec", repeated, tag="33")]
    pub signatures: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
/// Nested message and enum types in `Transaction`.
pub mod transaction {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Message {
        #[prost(oneof="message::Value", tags="100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111, 112, 113, 114, 115, 116, 117, 118, 119, 120, 121")]
        pub value: ::core::option::Option<message::Value>,
    }
    /// Nested message and enum types in `Message`.
    pub mod message {
        #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Value {
            #[prost(message, tag="100")]
            Other(::prost_types::Any),
            #[prost(message, tag="101")]
            MsgExec(super::super::super::super::super::cosmos::authz::v1beta1::MsgExec),
            #[prost(message, tag="102")]
            MsgSend(super::super::super::super::super::cosmos::bank::v1beta1::MsgSend),
            #[prost(message, tag="103")]
            MsgMultiSend(super::super::super::super::super::cosmos::bank::v1beta1::MsgMultiSend),
            #[prost(message, tag="104")]
            MsgVerifyInvariant(super::super::super::super::super::cosmos::crisis::v1beta1::MsgVerifyInvariant),
            #[prost(message, tag="105")]
            MsgWithdrawDelegatorReward(super::super::super::super::super::cosmos::distribution::v1beta1::MsgWithdrawDelegatorReward),
            #[prost(message, tag="106")]
            MsgWithdrawValidatorCommission(super::super::super::super::super::cosmos::distribution::v1beta1::MsgWithdrawValidatorCommission),
            #[prost(message, tag="107")]
            MsgSetWithdrawAddress(super::super::super::super::super::cosmos::distribution::v1beta1::MsgSetWithdrawAddress),
            #[prost(message, tag="108")]
            MsgFundCommunityPool(super::super::super::super::super::cosmos::distribution::v1beta1::MsgFundCommunityPool),
            #[prost(message, tag="109")]
            MsgSubmitEvidence(super::super::super::super::super::cosmos::evidence::v1beta1::MsgSubmitEvidence),
            #[prost(message, tag="110")]
            MsgSubmitProposal(super::super::super::super::super::cosmos::gov::v1beta1::MsgSubmitProposal),
            #[prost(message, tag="111")]
            MsgVote(super::super::super::super::super::cosmos::gov::v1beta1::MsgVote),
            #[prost(message, tag="112")]
            MsgDeposit(super::super::super::super::super::cosmos::gov::v1beta1::MsgDeposit),
            #[prost(message, tag="113")]
            MsgUnjail(super::super::super::super::super::cosmos::slashing::v1beta1::MsgUnjail),
            #[prost(message, tag="114")]
            MsgBid(super::super::super::super::super::injective::auction::v1beta1::MsgBid),
            #[prost(message, tag="115")]
            PubKey(super::super::super::super::super::injective::crypto::v1beta1::ethsecp256k1::PubKey),
            #[prost(message, tag="116")]
            PrivKey(super::super::super::super::super::injective::crypto::v1beta1::ethsecp256k1::PrivKey),
            #[prost(message, tag="117")]
            InjMsgDeposit(super::super::super::super::super::injective::exchange::v1beta1::MsgDeposit),
            #[prost(message, tag="118")]
            MsgRequestBatch(super::super::super::super::super::injective::peggy::v1::MsgRequestBatch),
            #[prost(message, tag="119")]
            MsgRegisterContract(super::super::super::super::super::injective::wasmx::v1::MsgRegisterContract),
            #[prost(message, tag="120")]
            MsgExecuteContractCompat(super::super::super::super::super::injective::wasmx::v1::MsgExecuteContractCompat),
            #[prost(message, tag="121")]
            MsgExecuteContract(super::super::super::super::super::cosmwasm::wasm::v1::MsgExecuteContract),
        }
    }
}
// @@protoc_insertion_point(module)
