// https://docs.rs/tower/latest/tower/trait.Service.html

use env_logger;
use std::env;
use webhook_handler::server::run_server;

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

    run_server(
        api_data_endpoint,
        kafka_message_queue_endpoint,
        kafka_sasl_username,
        kafka_sasl_password,
    )
    .await;
}
