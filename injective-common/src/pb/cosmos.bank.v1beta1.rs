// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Input {
    #[prost(string, tag="1")]
    pub address: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="2")]
    pub coins: ::prost::alloc::vec::Vec<super::super::base::v1beta1::Coin>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Output {
    #[prost(string, tag="1")]
    pub address: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="2")]
    pub coins: ::prost::alloc::vec::Vec<super::super::base::v1beta1::Coin>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSend {
    #[prost(string, tag="1")]
    pub from_address: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub to_address: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="3")]
    pub amount: ::prost::alloc::vec::Vec<super::super::base::v1beta1::Coin>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgMultiSend {
    #[prost(message, repeated, tag="1")]
    pub inputs: ::prost::alloc::vec::Vec<Input>,
    #[prost(message, repeated, tag="2")]
    pub outputs: ::prost::alloc::vec::Vec<Output>,
}
// @@protoc_insertion_point(module)
