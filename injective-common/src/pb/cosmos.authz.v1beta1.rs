// @generated
/// GenericAuthorization gives the grantee unrestricted permissions to execute
/// the provided method on behalf of the granter's account.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenericAuthorization {
    /// Msg, identified by it's type URL, to grant unrestricted permissions to execute
    #[prost(string, tag="1")]
    pub msg: ::prost::alloc::string::String,
}
/// Grant gives permissions to execute
/// the provide method with expiration time.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Grant {
    #[prost(message, optional, tag="1")]
    pub authorization: ::core::option::Option<::prost_types::Any>,
    /// time when the grant will expire and will be pruned. If null, then the grant
    /// doesn't have a time expiration (other conditions  in `authorization`
    /// may apply to invalidate the grant)
    #[prost(message, optional, tag="2")]
    pub expiration: ::core::option::Option<::prost_types::Timestamp>,
}
/// GrantAuthorization extends a grant with both the addresses of the grantee and granter.
/// It is used in genesis.proto and query.proto
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GrantAuthorization {
    #[prost(string, tag="1")]
    pub granter: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub grantee: ::prost::alloc::string::String,
    #[prost(message, optional, tag="3")]
    pub authorization: ::core::option::Option<::prost_types::Any>,
    #[prost(message, optional, tag="4")]
    pub expiration: ::core::option::Option<::prost_types::Timestamp>,
}
/// GrantQueueItem contains the list of TypeURL of a sdk.Msg.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GrantQueueItem {
    /// msg_type_urls contains the list of TypeURL of a sdk.Msg.
    #[prost(string, repeated, tag="1")]
    pub msg_type_urls: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// MsgGrant is a request type for Grant method. It declares authorization to the grantee
/// on behalf of the granter with the provided expiration time.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgGrant {
    #[prost(string, tag="1")]
    pub granter: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub grantee: ::prost::alloc::string::String,
    #[prost(message, optional, tag="3")]
    pub grant: ::core::option::Option<Grant>,
}
/// MsgGrantResponse defines the Msg/MsgGrant response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgGrantResponse {
}
/// MsgExec attempts to execute the provided messages using
/// authorizations granted to the grantee. Each message should have only
/// one signer corresponding to the granter of the authorization.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgExec {
    #[prost(string, tag="1")]
    pub grantee: ::prost::alloc::string::String,
    /// Execute Msg.
    /// The x/authz will try to find a grant matching (msg.signers\[0\], grantee, MsgTypeURL(msg))
    /// triple and validate it.
    #[prost(message, repeated, tag="2")]
    pub msgs: ::prost::alloc::vec::Vec<::prost_types::Any>,
}
/// MsgExecResponse defines the Msg/MsgExecResponse response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgExecResponse {
    #[prost(bytes="vec", repeated, tag="1")]
    pub results: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
/// MsgRevoke revokes any authorization with the provided sdk.Msg type on the
/// granter's account with that has been granted to the grantee.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgRevoke {
    #[prost(string, tag="1")]
    pub granter: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub grantee: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub msg_type_url: ::prost::alloc::string::String,
}
/// MsgRevokeResponse defines the Msg/MsgRevokeResponse response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgRevokeResponse {
}
/// MsgPruneExpiredGrants prunes the expired grants.
///
/// Since x/authz v1.0.0
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgPruneExpiredGrants {
    #[prost(string, tag="1")]
    pub pruner: ::prost::alloc::string::String,
}
/// MsgPruneExpiredGrantsResponse defines the Msg/MsgPruneExpiredGrantsResponse response type.
///
/// Since x/authz v1.0.0
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgPruneExpiredGrantsResponse {
}
// @@protoc_insertion_point(module)
