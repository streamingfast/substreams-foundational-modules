// @generated
pub mod sf {
    pub mod antelope {
        pub mod r#type {
            // @@protoc_insertion_point(attribute:sf.antelope.type.v1)
            pub mod v1 {
                include!("sf.antelope.type.v1.rs");
                // @@protoc_insertion_point(sf.antelope.type.v1)
            }
        }
    }
    // @@protoc_insertion_point(attribute:sf.substreams)
    pub mod substreams {
        include!("sf.substreams.rs");
        // @@protoc_insertion_point(sf.substreams)
        pub mod antelope {
            // @@protoc_insertion_point(attribute:sf.substreams.antelope.v1)
            pub mod v1 {
                include!("sf.substreams.antelope.v1.rs");
                // @@protoc_insertion_point(sf.substreams.antelope.v1)
            }
        }
        pub mod index {
            // @@protoc_insertion_point(attribute:sf.substreams.index.v1)
            pub mod v1 {
                include!("sf.substreams.index.v1.rs");
                // @@protoc_insertion_point(sf.substreams.index.v1)
            }
        }
        // @@protoc_insertion_point(attribute:sf.substreams.v1)
        pub mod v1 {
            include!("sf.substreams.v1.rs");
            // @@protoc_insertion_point(sf.substreams.v1)
        }
    }
}
