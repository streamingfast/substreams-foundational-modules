// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UsdtExchange {
    #[prost(int32, tag="1")]
    pub amount: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UsdtExchangeList {
    #[prost(message, repeated, tag="1")]
    pub exchanges: ::prost::alloc::vec::Vec<UsdtExchange>,
}
// @@protoc_insertion_point(module)
