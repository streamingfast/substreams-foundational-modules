// @generated
/// ValidatorSigningInfo defines a validator's signing info for monitoring their
/// liveness activity.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValidatorSigningInfo {
    #[prost(string, tag="1")]
    pub address: ::prost::alloc::string::String,
    /// Height at which validator was first a candidate OR was un-jailed
    #[prost(int64, tag="2")]
    pub start_height: i64,
    /// DEPRECATED: Index which is incremented every time a validator is bonded in a block and
    /// _may_ have signed a pre-commit or not. This in conjunction with the
    /// signed_blocks_window param determines the index in the missed block bitmap.
    #[deprecated]
    #[prost(int64, tag="3")]
    pub index_offset: i64,
    /// Timestamp until which the validator is jailed due to liveness downtime.
    #[prost(message, optional, tag="4")]
    pub jailed_until: ::core::option::Option<::prost_types::Timestamp>,
    /// Whether or not a validator has been tombstoned (killed out of validator
    /// set). It is set once the validator commits an equivocation or for any other
    /// configured misbehavior.
    #[prost(bool, tag="5")]
    pub tombstoned: bool,
    /// A counter of missed (unsigned) blocks. It is used to avoid unnecessary
    /// reads in the missed block bitmap.
    #[prost(int64, tag="6")]
    pub missed_blocks_counter: i64,
}
/// Params represents the parameters used for by the slashing module.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Params {
    #[prost(int64, tag="1")]
    pub signed_blocks_window: i64,
    #[prost(bytes="vec", tag="2")]
    pub min_signed_per_window: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag="3")]
    pub downtime_jail_duration: ::core::option::Option<::prost_types::Duration>,
    #[prost(bytes="vec", tag="4")]
    pub slash_fraction_double_sign: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="5")]
    pub slash_fraction_downtime: ::prost::alloc::vec::Vec<u8>,
}
/// MsgUnjail defines the Msg/Unjail request type
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUnjail {
    #[prost(string, tag="1")]
    pub validator_addr: ::prost::alloc::string::String,
}
/// MsgUnjailResponse defines the Msg/Unjail response type
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUnjailResponse {
}
/// MsgUpdateParams is the Msg/UpdateParams request type.
///
/// Since: cosmos-sdk 0.47
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateParams {
    /// authority is the address that controls the module (defaults to x/gov unless overwritten).
    #[prost(string, tag="1")]
    pub authority: ::prost::alloc::string::String,
    /// params defines the x/slashing parameters to update.
    ///
    /// NOTE: All parameters must be supplied.
    #[prost(message, optional, tag="2")]
    pub params: ::core::option::Option<Params>,
}
/// MsgUpdateParamsResponse defines the response structure for executing a
/// MsgUpdateParams message.
///
/// Since: cosmos-sdk 0.47
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateParamsResponse {
}
// @@protoc_insertion_point(module)
