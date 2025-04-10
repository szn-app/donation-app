use axum::{
    extract::{Form, Query},
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::{get, post},
    Extension, Json, Router,
};
use rdkafka::producer::{FutureProducer, FutureRecord};
use serde::{Deserialize, Serialize};
use serde_json::json;
use shared_proto::proto::user_sync::{AddUserRequest, AddUserResponse, UserSyncClient};
use std::net::SocketAddr;
use std::time::Duration;
use tonic::transport::Channel;

use crate::server::AppConfig;

#[derive(Debug, Serialize, Deserialize)]
pub struct WebhookPayloadRegistrationAfter {
    user_id: Option<String>,
}

pub async fn webhook_kratos_registration_handler(
    Extension(app_config): Extension<AppConfig>,
    Extension(kafka_producer): Extension<FutureProducer>,
    Extension(grpc_channel): Extension<Channel>,
    Json(payload): Json<WebhookPayloadRegistrationAfter>,
) -> impl IntoResponse {
    log::debug!(
        "Received webhook data: {:?}",
        payload.user_id.as_deref().unwrap_or_default()
    );

    match kafka_send_topic_message(kafka_producer, &payload).await {
        Ok(()) => log::debug!("Message sent successfully"),
        Err(e) => {
            log::error!("Failed to send Kafka message: {}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": format!("Failed to send Kafka message: {}", e) })),
            )
                .into_response();
        }
    }

    // gRPC call to add user into the database
    match grpc_add_user(grpc_channel, &payload).await {
        Ok(_) => log::debug!("User added successfully"),
        Err(e) => {
            log::error!("Failed to add user id: {}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": format!("Failed to add user id: {}", e) })),
            )
                .into_response();
        }
    }

    (StatusCode::OK, Json(payload)).into_response()
}

async fn kafka_send_topic_message(
    kafka_producer: FutureProducer,
    payload: &WebhookPayloadRegistrationAfter,
) -> Result<(), rdkafka::error::KafkaError> {
    let user_id = payload.user_id.as_ref().unwrap();
    let delivery_status = kafka_producer
        .send::<_, _, _>(
            FutureRecord::to(&"kratos-user-registered")
                .key("user_id")
                .payload(user_id),
            Duration::from_secs(5), // Timeout
        )
        .await;

    match delivery_status {
        Ok(_) => {
            log::debug!("Message sent to Kafka");
            Ok(())
        }
        Err(e) => {
            log::error!("Failed to send message to Kafka: {:?}", e);
            Err(e.0)
        }
    }
}

async fn grpc_add_user(
    grpc_channel: Channel,
    payload: &WebhookPayloadRegistrationAfter,
) -> Result<AddUserResponse, tonic::Status> {
    let user_id = payload.user_id.as_ref().unwrap();

    // channel cheaply cloned, bc internally uses  reference count
    let mut client = UserSyncClient::new(grpc_channel);

    let request = tonic::Request::new(AddUserRequest {
        user_id: user_id.to_string(),
    });
    let response = client.add_user(request).await?.into_inner();

    Ok(response)
}
