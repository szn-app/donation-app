use tonic::{
    transport::Server as TonicServer, Request as TonicRequest, Response as TonicResponse, Status,
};

// import generated gRPC proto-rust file into module types/interfaces

mod keto_opl_service {
    tonic::include_proto!("ory.keto.opl.v1alpha1");
}

mod keto_relation_tuples_service {
    tonic::include_proto!("ory.keto.relation_tuples.v1alpha2");
}

// Re-export all generated code into a single module tree for convenience
pub mod proto {
    pub mod opl {
        pub use super::super::keto_opl_service::*;
        // syntax_service
        pub const DESCRIPTOR_SET: &[u8] =
            include_bytes!("protobuf-autogen/syntax_service_descriptor.bin");
    }

    pub mod relation_tuples {
        pub use super::super::keto_relation_tuples_service::*;

        // Combined list of all descriptor sets
        pub const DESCRIPTOR_SET_LIST: &[&[u8]] = &[
            include_bytes!("protobuf-autogen/check_service_descriptor.bin"),
            include_bytes!("protobuf-autogen/expand_service_descriptor.bin"),
            include_bytes!("protobuf-autogen/namespaces_service_descriptor.bin"),
            include_bytes!("protobuf-autogen/read_service_descriptor.bin"),
            include_bytes!("protobuf-autogen/relation_tuples_descriptor.bin"),
            include_bytes!("protobuf-autogen/version_descriptor.bin"),
            include_bytes!("protobuf-autogen/write_service_descriptor.bin"),
        ];
    }
}
