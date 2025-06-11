use crate::server::connection::PostgresPool;
use log;
use std::error::Error;
use std::net;

/// Run the gRPC server
pub async fn start_grpc_server(postgres_pool_group: PostgresPool) -> Result<(), Box<dyn Error>> {
    let grpc_addr: net::SocketAddr = "0.0.0.0:8082".parse()?;
    let server = crate::api::grpc::configure_grpc_server(postgres_pool_group)?;

    log::info!("gRPC server running on http://{}", grpc_addr);
    server.serve(grpc_addr).await?;

    Ok(())
}
