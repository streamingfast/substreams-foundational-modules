// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TxList {
    #[prost(message, repeated, tag="1")]
    pub transactions: ::prost::alloc::vec::Vec<super::super::super::super::cosmos::tx::v1beta1::Tx>,
}
// @@protoc_insertion_point(module)
