use tokio;

pub mod connection;
pub mod grpc;
pub mod http;

pub async fn run_server(
    config: connection::PostgresEndpointConfig,
) -> Result<(), Box<dyn std::error::Error>> {
    let postgres_pool_group = {
        let db_context = connection::DatabaseContext::new(&config.rw, &config.ro, &config.r)
            .await
            .expect("Failed to create database context");

        db_context
            .test_connection()
            .await
            .expect("Database pool connections failed");

        db_context.postgres_pool_group
    };

    tokio::select! {
        result = http::start_http_server(postgres_pool_group.clone()) => result,
        result = grpc::start_grpc_server(postgres_pool_group) => result
    }
}
