use tokio;

pub mod connection;
pub mod grpc;
pub mod http;

pub async fn run_server(
    (pg_config, keto_config, app_endpoint): (
        connection::PostgresEndpointConfig,
        connection::KetoEndpointConfig,
        String,
    ),
) -> Result<(), Box<dyn std::error::Error>> {
    let postgres_pool_group = {
        let db_context = connection::postgresql::DatabaseContext::new(
            &pg_config.rw,
            &pg_config.ro,
            &pg_config.r,
        )
        .await
        .expect("Failed to create database context");

        db_context
            .test_connection()
            .await
            .expect("Database pool connections failed");

        db_context.postgres_pool_group
    };

    // create keto client grpc connection
    let keto_channel_group = {
        connection::keto::KetoChannelGroup::new(&keto_config.read, &keto_config.write)
            .await
            .expect("Failed to create gRPC channel")
    };

    tokio::select! {
        result = http::start_http_server(postgres_pool_group.clone(), keto_channel_group, &app_endpoint) => result,
        result = grpc::start_grpc_server(postgres_pool_group) => result
    }
}
