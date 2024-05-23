// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CompactBitArray {
    #[prost(uint32, tag="1")]
    pub extra_bits_stored: u32,
    #[prost(bytes="vec", tag="2")]
    pub elems: ::prost::alloc::vec::Vec<u8>,
}
// @@protoc_insertion_point(module)
