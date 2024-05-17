// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Transactions {
    /// sf.substreams.v1.Clock clock = 2;
    #[prost(message, repeated, tag="1")]
    pub transactions: ::prost::alloc::vec::Vec<Transaction>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Transaction {
    #[prost(message, optional, tag="1")]
    pub trace: ::core::option::Option<::substreams_antelope::pb::TransactionTrace>,
    #[prost(string, tag="2")]
    pub tx_hash: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Actions {
    /// sf.substreams.v1.Clock clock = 2;
    #[prost(message, repeated, tag="1")]
    pub actions: ::prost::alloc::vec::Vec<Action>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Action {
    #[prost(message, optional, tag="1")]
    pub trace: ::core::option::Option<::substreams_antelope::pb::ActionTrace>,
    #[prost(string, tag="2")]
    pub tx_hash: ::prost::alloc::string::String,
}
// @@protoc_insertion_point(module)
