// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Tx {
    #[prost(message, optional, tag="1")]
    pub body: ::core::option::Option<TxBody>,
    #[prost(message, optional, tag="2")]
    pub auth_info: ::core::option::Option<AuthInfo>,
    #[prost(bytes="vec", repeated, tag="3")]
    pub signatures: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TxBody {
    #[prost(message, repeated, tag="1")]
    pub messages: ::prost::alloc::vec::Vec<::prost_types::Any>,
    #[prost(string, tag="2")]
    pub memo: ::prost::alloc::string::String,
    #[prost(uint64, tag="3")]
    pub timeout_height: u64,
    #[prost(message, repeated, tag="1023")]
    pub extension_options: ::prost::alloc::vec::Vec<::prost_types::Any>,
    #[prost(message, repeated, tag="2047")]
    pub non_critical_extension_options: ::prost::alloc::vec::Vec<::prost_types::Any>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthInfo {
    #[prost(message, repeated, tag="1")]
    pub signer_infos: ::prost::alloc::vec::Vec<SignerInfo>,
    #[prost(message, optional, tag="2")]
    pub fee: ::core::option::Option<Fee>,
    #[deprecated]
    #[prost(message, optional, tag="3")]
    pub tip: ::core::option::Option<Tip>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SignerInfo {
    #[prost(message, optional, tag="1")]
    pub public_key: ::core::option::Option<::prost_types::Any>,
    #[prost(message, optional, tag="2")]
    pub mode_info: ::core::option::Option<ModeInfo>,
    #[prost(uint64, tag="3")]
    pub sequence: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModeInfo {
    #[prost(oneof="mode_info::Sum", tags="1, 2")]
    pub sum: ::core::option::Option<mode_info::Sum>,
}
/// Nested message and enum types in `ModeInfo`.
pub mod mode_info {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Single {
        #[prost(enumeration="super::super::signing::v1beta1::SignMode", tag="1")]
        pub mode: i32,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Multi {
        #[prost(message, optional, tag="1")]
        pub bitarray: ::core::option::Option<super::super::super::crypto::multisig::v1beta1::CompactBitArray>,
        #[prost(message, repeated, tag="2")]
        pub mode_infos: ::prost::alloc::vec::Vec<super::ModeInfo>,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Sum {
        #[prost(message, tag="1")]
        Single(Single),
        #[prost(message, tag="2")]
        Multi(Multi),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Fee {
    #[prost(message, repeated, tag="1")]
    pub amount: ::prost::alloc::vec::Vec<super::super::base::v1beta1::Coin>,
    #[prost(uint64, tag="2")]
    pub gas_limit: u64,
    #[prost(string, tag="3")]
    pub payer: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub granter: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Tip {
    #[prost(message, repeated, tag="1")]
    pub amount: ::prost::alloc::vec::Vec<super::super::base::v1beta1::Coin>,
    #[prost(string, tag="2")]
    pub tipper: ::prost::alloc::string::String,
}
// @@protoc_insertion_point(module)
