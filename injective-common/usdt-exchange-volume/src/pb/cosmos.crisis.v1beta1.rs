// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgVerifyInvariant {
    #[prost(string, tag="1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub invariant_module_name: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub invariant_route: ::prost::alloc::string::String,
}
// @@protoc_insertion_point(module)
