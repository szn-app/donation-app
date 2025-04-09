use axum::{
    extract::{Form, Query},
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::{get, post},
    Extension, Json, Router,
};
use log;
use rdkafka::config::ClientConfig;
use rdkafka::producer::{FutureProducer, FutureRecord};
use serde::{Deserialize, Serialize};
use std::env;
use std::net::SocketAddr;
use std::time::Duration;

#[derive(Clone)]
pub struct AppConfig {
    pub api_data_endpoint: String,
    pub kafka_message_queue_endpoint: String,
    pub kafka_sasl_username: String,
    pub kafka_sasl_password: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct WebhookPayloadRegistrationAfter {
    user_id: Option<String>,
}

pub fn routes() -> Router {
    Router::new()
        .route("/health/status", get(health_status))
        .route(
            "/auth/kratos/registration",
            post(webhook_kratos_registration_handler),
        )
        .route("/auth/kratos/login", post(webhook_kratos_handler_debug))
        .fallback(handler_404)
}

pub async fn handler_404() -> impl IntoResponse {
    log::info!("--> fallback 404 handler called");

    (
        StatusCode::NOT_FOUND,
        Html("<h1>404</h1><p>Nothing to see here</p>"),
    )
}

pub async fn health_status() -> impl IntoResponse {
    log::info!("--> Health status endpoint called");

    Html(format!("ok")).into_response()
}

pub async fn webhook_kratos_handler_debug(
    Json(payload): Json<serde_json::Value>,
) -> impl IntoResponse {
    log::info!("Received webhook data: {:?}", payload);

    (StatusCode::OK, Json(payload))
}

pub async fn webhook_kratos_registration_handler(
    Extension(app_config): Extension<AppConfig>,
    Extension(kafka_producer): Extension<FutureProducer>,
    Json(payload): Json<WebhookPayloadRegistrationAfter>,
) -> impl IntoResponse {
    log::debug!(
        "Received webhook data: {:?}",
        payload.user_id.as_deref().unwrap_or_default()
    );

    // send kafka topic message
    {
        let delivery_status = kafka_producer
            .send::<_, _, _>(
                FutureRecord::to(&"kratos-user-registered")
                    .key("user_id")
                    .payload(&payload.user_id.as_ref().unwrap()),
                Duration::from_secs(5), // Timeout
            )
            .await;

        match delivery_status {
            Ok(_) => log::debug!("Message sent to Kafka"),
            Err(e) => log::error!("Failed to send message to Kafka: {:?}", e),
        }
    }

    // gRPC/REST call to add user into the database
    {}

    (StatusCode::OK, Json(payload))
}

pub fn create_kafka_producer(endpoint: &str, username: &str, password: &str) -> FutureProducer {
    let mut kafka_config = ClientConfig::new();

    // https://github.com/confluentinc/librdkafka/blob/master/CONFIGURATION.md
    kafka_config
        .set("bootstrap.servers", endpoint)
        .set("security.protocol", "sasl_plaintext")
        .set("sasl.mechanism", "SCRAM-SHA-512") // Matches your JAAS config
        .set("sasl.username", username)
        .set("sasl.password", password);

    kafka_config
        .create()
        .expect("Failed to create Kafka producer")
}
