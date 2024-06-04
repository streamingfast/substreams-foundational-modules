// @generated
/// MsgSubmitEvidence represents a message that supports submitting arbitrary
/// Evidence of misbehavior such as equivocation or counterfactual signing.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSubmitEvidence {
    /// submitter is the signer account address of evidence.
    #[prost(string, tag="1")]
    pub submitter: ::prost::alloc::string::String,
    /// evidence defines the evidence of misbehavior.
    #[prost(message, optional, tag="2")]
    pub evidence: ::core::option::Option<::prost_types::Any>,
}
/// MsgSubmitEvidenceResponse defines the Msg/SubmitEvidence response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSubmitEvidenceResponse {
    /// hash defines the hash of the evidence.
    #[prost(bytes="vec", tag="4")]
    pub hash: ::prost::alloc::vec::Vec<u8>,
}
// @@protoc_insertion_point(module)
