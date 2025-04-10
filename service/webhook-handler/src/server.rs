use axum;
use log;
use rdkafka::config::ClientConfig;
use rdkafka::error::KafkaResult;
use rdkafka::producer::FutureProducer;

use tonic::transport::Channel;

pub mod router;

#[derive(Clone)]
pub struct AppConfig {
    pub api_data_endpoint: String,
    pub kafka_message_queue_endpoint: String,
    pub kafka_sasl_username: String,
    pub kafka_sasl_password: String,
}

pub fn create_kafka_producer(
    endpoint: &str,
    username: &str,
    password: &str,
) -> KafkaResult<FutureProducer> {
    let mut kafka_config = ClientConfig::new();

    // https://github.com/confluentinc/librdkafka/blob/master/CONFIGURATION.md
    kafka_config
        .set("bootstrap.servers", endpoint)
        .set("security.protocol", "sasl_plaintext")
        .set("sasl.mechanism", "SCRAM-SHA-512") // Matches your JAAS config
        .set("sasl.username", username)
        .set("sasl.password", password);

    kafka_config.create()
}

pub async fn create_grpc_channel(
    address: &str,
) -> Result<Channel, Box<dyn std::error::Error + Send + Sync>> {
    let channel = Channel::from_shared(address.to_string())?.connect().await?;
    Ok(channel)
}

pub async fn run_server(
    api_data_endpoint: String,
    kafka_message_queue_endpoint: String,
    kafka_sasl_username: String,
    kafka_sasl_password: String,
) {
    let app_config = AppConfig {
        api_data_endpoint,
        kafka_message_queue_endpoint,
        kafka_sasl_username,
        kafka_sasl_password,
    };

    // Create Kafka producer with SASL configuration
    let kafka_producer = create_kafka_producer(
        &app_config.kafka_message_queue_endpoint,
        &app_config.kafka_sasl_username,
        &app_config.kafka_sasl_password,
    )
    .expect("Failed to create Kafka producer");

    let grpc_channel = create_grpc_channel(&app_config.api_data_endpoint)
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
