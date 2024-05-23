// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSubmitEvidence {
    #[prost(string, tag="1")]
    pub submitter: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub evidence: ::core::option::Option<::prost_types::Any>,
}
// @@protoc_insertion_point(module)
