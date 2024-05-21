// @generated
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SignMode {
    Unspecified = 0,
    Direct = 1,
    Textual = 2,
    DirectAux = 3,
    LegacyAminoJson = 127,
    Eip191 = 191,
}
impl SignMode {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            SignMode::Unspecified => "SIGN_MODE_UNSPECIFIED",
            SignMode::Direct => "SIGN_MODE_DIRECT",
            SignMode::Textual => "SIGN_MODE_TEXTUAL",
            SignMode::DirectAux => "SIGN_MODE_DIRECT_AUX",
            SignMode::LegacyAminoJson => "SIGN_MODE_LEGACY_AMINO_JSON",
            SignMode::Eip191 => "SIGN_MODE_EIP_191",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "SIGN_MODE_UNSPECIFIED" => Some(Self::Unspecified),
            "SIGN_MODE_DIRECT" => Some(Self::Direct),
            "SIGN_MODE_TEXTUAL" => Some(Self::Textual),
            "SIGN_MODE_DIRECT_AUX" => Some(Self::DirectAux),
            "SIGN_MODE_LEGACY_AMINO_JSON" => Some(Self::LegacyAminoJson),
            "SIGN_MODE_EIP_191" => Some(Self::Eip191),
            _ => None,
        }
    }
}
// @@protoc_insertion_point(module)
