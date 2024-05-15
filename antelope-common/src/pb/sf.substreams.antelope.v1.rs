// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Transactions {
    #[prost(message, repeated, tag = "1")]
    pub transactions: ::prost::alloc::vec::Vec<Transaction>,
    #[prost(message, optional, tag = "2")]
    pub clock: ::core::option::Option<super::super::v1::Clock>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Transaction {
    #[prost(message, optional, tag = "1")]
    pub trace: ::core::option::Option<super::super::super::antelope::r#type::v1::TransactionTrace>,
    #[prost(string, tag = "2")]
    pub tx_hash: ::prost::alloc::string::String,
}
// @@protoc_insertion_point(module)
