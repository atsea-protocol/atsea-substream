// @generated
pub mod contract {
    // @@protoc_insertion_point(attribute:contract.v1)
    pub mod v1 {
        include!("contract.v1.rs");
        // @@protoc_insertion_point(contract.v1)
    }
}
pub mod sf {
    pub mod ethereum {
        pub mod r#type {
            // @@protoc_insertion_point(attribute:sf.ethereum.type.v2)
            pub mod v2 {
                include!("sf.ethereum.type.v2.rs");
                // @@protoc_insertion_point(sf.ethereum.type.v2)
            }
        }
        pub mod substreams {
            // @@protoc_insertion_point(attribute:sf.ethereum.substreams.v1)
            pub mod v1 {
                include!("sf.ethereum.substreams.v1.rs");
                // @@protoc_insertion_point(sf.ethereum.substreams.v1)
            }
        }
    }
    pub mod substreams {
        pub mod sink {
            pub mod pubsub {
                // @@protoc_insertion_point(attribute:sf.substreams.sink.pubsub.v1)
                pub mod v1 {
                    include!("sf.substreams.sink.pubsub.v1.rs");
                    // @@protoc_insertion_point(sf.substreams.sink.pubsub.v1)
                }
            }
        }
    }
}
