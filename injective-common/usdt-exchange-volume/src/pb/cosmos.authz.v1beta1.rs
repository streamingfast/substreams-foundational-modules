// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgExec {
    #[prost(string, tag="1")]
    pub grantee: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="2")]
    pub msgs: ::prost::alloc::vec::Vec<::prost_types::Any>,
}
// @@protoc_insertion_point(module)
