use axum;
use log;

pub mod context;
pub mod router;

pub async fn run_server(
    api_data_endpoint: String,
    kafka_message_queue_endpoint: String,
    kafka_sasl_username: String,
    kafka_sasl_password: String,
) {
    let app_config = context::AppConfig {
        api_data_endpoint,
        kafka_message_queue_endpoint,
        kafka_sasl_username,
        kafka_sasl_password,
    };

    // Create Kafka producer with SASL configuration
    let kafka_producer = context::create_kafka_producer(
        &app_config.kafka_message_queue_endpoint,
        &app_config.kafka_sasl_username,
        &app_config.kafka_sasl_password,
    )
    .expect("Failed to create Kafka producer");

    let grpc_channel = context::create_grpc_channel(&app_config.api_data_endpoint)
        .await
        .expect("Failed to create gRPC channel");

    let app = router::routes()
        .layer(axum::Extension(app_config))
        .layer(axum::Extension(grpc_channel))
        .layer(axum::Extension(kafka_producer));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3010").await.unwrap();
    log::info!("Server running on http://0.0.0.0:3010");
    axum::serve(listener, app).await.unwrap();
}
