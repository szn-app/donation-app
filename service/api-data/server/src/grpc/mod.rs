use crate::server::connection::PostgresPool;
use shared_proto::proto;
use std::error::Error;
use tonic::transport::Server as TonicServer;
use tonic_reflection::server::Builder as ReflectionBuilder;

pub mod handler;

pub async fn start_grpc_server(
    postgres_pool_group: PostgresPool,
) -> Result<(), Box<dyn std::error::Error>> {
    let grpc_addr = "0.0.0.0:8082".parse()?;

    let reflection_service = ReflectionBuilder::configure()
        .register_encoded_file_descriptor_set(proto::user_sync::DESCRIPTOR_SET)
        .register_encoded_file_descriptor_set(proto::test::DESCRIPTOR_SET)
        .build_v1()?;

    log::info!("gRPC server running on http://{}", grpc_addr);

    TonicServer::builder()
        .add_service(proto::user_sync::UserSyncServer::new(
            handler::UserSyncService::new(postgres_pool_group.clone()),
        ))
        .add_service(proto::test::GreeterServer::new(proto::test::GreeterService))
        .add_service(reflection_service)
        .serve(grpc_addr)
        .await?;

    Ok(())
}
