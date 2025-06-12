use tonic::{
    transport::Server as TonicServer, Request as TonicRequest, Response as TonicResponse, Status,
};

// import generated gRPC proto-rust file into module types/interfaces
mod proto_opl {
    tonic::include_proto!("ory.keto.opl.v1alpha1"); // relies internally on `prost`
}
mod proto_relation_tuples {
    tonic::include_proto!("ory.keto.relation_tuples.v1alpha2"); // relies internally on `prost`
}

// Re-export all generated code into a single module tree for convenience
pub mod proto {
    pub mod opl {
        pub use super::super::proto_opl::*;

        pub const DESCRIPTOR_SET: &[u8] =
            include_bytes!("protobuf-autogen/ory_keto_opl_v1alpha1_descriptor.bin");
    }

    // import generated gRPC proto-rust file into module types/interfaces
    pub mod relation_tuples {
        pub use super::super::proto_relation_tuples::*;

        // Combined list of all descriptor sets
        pub const DESCRIPTOR_SET: &[u8] =
            include_bytes!("protobuf-autogen/ory_keto_relation_tuples_v1alpha2_descriptor.bin");
    }
}
