// @generated
pub mod evm {
    pub mod erc20 {
        pub mod metadata {
            // @@protoc_insertion_point(attribute:evm.erc20.metadata.v1)
            pub mod v1 {
                include!("evm.erc20.metadata.v1.rs");
                // @@protoc_insertion_point(evm.erc20.metadata.v1)
            }
        }
        pub mod stores {
            // @@protoc_insertion_point(attribute:evm.erc20.stores.v1)
            pub mod v1 {
                include!("evm.erc20.stores.v1.rs");
                // @@protoc_insertion_point(evm.erc20.stores.v1)
            }
        }
        pub mod transfers {
            // @@protoc_insertion_point(attribute:evm.erc20.transfers.v1)
            pub mod v1 {
                include!("evm.erc20.transfers.v1.rs");
                // @@protoc_insertion_point(evm.erc20.transfers.v1)
            }
        }
    }
}
pub mod sf {
    // @@protoc_insertion_point(attribute:sf.substreams)
    pub mod substreams {
        include!("sf.substreams.rs");
        // @@protoc_insertion_point(sf.substreams)
        pub mod ethereum {
            pub mod erc20 {
                // @@protoc_insertion_point(attribute:sf.substreams.ethereum.erc20.v1)
                pub mod v1 {
                    include!("sf.substreams.ethereum.erc20.v1.rs");
                    // @@protoc_insertion_point(sf.substreams.ethereum.erc20.v1)
                }
            }
        }
    }
}
