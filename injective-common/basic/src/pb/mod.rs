// @generated
// @@protoc_insertion_point(attribute:amino)
pub mod cosmos {
    pub mod base {
        // @@protoc_insertion_point(attribute:cosmos.base.v1beta1)
        pub mod v1beta1 {
            include!("cosmos.base.v1beta1.rs");
            // @@protoc_insertion_point(cosmos.base.v1beta1)
        }
    }
    pub mod crypto {
        pub mod multisig {
            // @@protoc_insertion_point(attribute:cosmos.crypto.multisig.v1beta1)
            pub mod v1beta1 {
                include!("cosmos.crypto.multisig.v1beta1.rs");
                // @@protoc_insertion_point(cosmos.crypto.multisig.v1beta1)
            }
        }
    }
    pub mod tx {
        pub mod signing {
            // @@protoc_insertion_point(attribute:cosmos.tx.signing.v1beta1)
            pub mod v1beta1 {
                include!("cosmos.tx.signing.v1beta1.rs");
                // @@protoc_insertion_point(cosmos.tx.signing.v1beta1)
            }
        }
        // @@protoc_insertion_point(attribute:cosmos.tx.v1beta1)
        pub mod v1beta1 {
            include!("cosmos.tx.v1beta1.rs");
            // @@protoc_insertion_point(cosmos.tx.v1beta1)
        }
    }
}
// @@protoc_insertion_point(attribute:cosmos_proto)
// @@protoc_insertion_point(attribute:gogoproto)
pub mod sf {
    pub mod cosmos {
        pub mod r#type {
            // @@protoc_insertion_point(attribute:sf.cosmos.type.v2)
            pub mod v2 {
                include!("sf.cosmos.type.v2.rs");
                // @@protoc_insertion_point(sf.cosmos.type.v2)
            }
        }
    }
    pub mod substreams {
        pub mod cosmos {
            // @@protoc_insertion_point(attribute:sf.substreams.cosmos.v1)
            pub mod v1 {
                include!("sf.substreams.cosmos.v1.rs");
                // @@protoc_insertion_point(sf.substreams.cosmos.v1)
            }
        }
    }
}
