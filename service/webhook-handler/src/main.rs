// https://docs.rs/tower/latest/tower/trait.Service.html

mod handler;

use axum;
use env_logger;
use handler::{create_kafka_producer, AppConfig};
use log;
use rdkafka::producer::FutureProducer;
use std::env;

#[tokio::main]
async fn main() {
    // Initialize logger with appropriate level based on environment
    let log_level = if cfg!(debug_assertions) {
        "debug"
    } else {
        "info"
    };
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or(log_level)).init();

    let api_data_endpoint: String =
        env::var("API_DATA_ENDPOINT").expect("API_DATA_ENDPOINT env not set");
    let kafka_message_queue_endpoint =
        env::var("KAFKA_MESSAGE_QUEUE_ENDPOINT").expect("KAFKA_MESSAGE_QUEUE_ENDPOINT env not set");
    let kafka_sasl_username =
        env::var("KAFKA_SASL_USERNAME").expect("KAFKA_SASL_USERNAME env not set");
    let kafka_sasl_password =
        env::var("KAFKA_SASL_PASSWORD").expect("KAFKA_SASL_PASSWORD env not set");

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
    );

    let app = handler::routes()
        .layer(axum::Extension(app_config))
        .layer(axum::Extension(kafka_producer));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3010").await.unwrap();
    log::info!("Server running on http://0.0.0.0:3010");
    axum::serve(listener, app).await.unwrap();
}
