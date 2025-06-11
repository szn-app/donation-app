use crate::server::connection::PostgresPool;
use axum::Router;
use shared_protobuf_webhook::proto;
use std::error::Error;
use tonic::transport::{server::Router as TonicRouter, Server as TonicServer};
use tonic_reflection::server::Builder as ReflectionBuilder;

pub mod handler;

/// Creates the gRPC server with all registered services
pub fn configure_grpc_server(
    postgres_pool_group: PostgresPool,
) -> Result<TonicRouter, Box<dyn Error>> {
    let reflection_service = ReflectionBuilder::configure()
        .register_encoded_file_descriptor_set(proto::user_sync::DESCRIPTOR_SET)
        .register_encoded_file_descriptor_set(proto::test::DESCRIPTOR_SET)
        .build_v1()?;

    let server = TonicServer::builder()
        .add_service(proto::user_sync::UserSyncServer::new(
            handler::UserSyncService::new(postgres_pool_group.clone()),
        ))
        .add_service(proto::test::GreeterServer::new(proto::test::GreeterService))
        .add_service(reflection_service);

    Ok(server)
}
