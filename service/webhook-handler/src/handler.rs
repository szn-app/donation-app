use axum::{
    extract::{Form, Query},
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::{get, post},
    Extension, Json, Router,
};
use log;
use serde::{Deserialize, Serialize};
use std::env;
use std::net::SocketAddr;

#[derive(Debug, Serialize, Deserialize)]
struct WebhookPayloadRegistrationAfter {
    user_id: Option<String>,
}

pub fn routes() -> Router {
    Router::new()
        .route("/health/status", get(health_status))
        .route("/webhook/auth/kratos", post(webhook_kratos_handler))
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

pub async fn webhook_kratos_handler_2(Json(payload): Json<WebhookPayload>) -> impl IntoResponse {
    log::info!("Received webhook data: {:?}", payload); // Log the received payload
    println!("executed webhook kratos");

    StatusCode::OK
}

// TODO:
pub async fn webhook_kratos_handler(
    Json(payload): Json<WebhookPayloadRegistrationAfter>,
) -> impl IntoResponse {
    log::debug!(
        "Received webhook data: {:?}",
        payload.user_id.as_deref().unwrap_or_default()
    );

    (StatusCode::OK, Json(payload))
}

pub async fn webhook_kratos_handler_debug(
    Json(payload): Json<serde_json::Value>,
) -> impl IntoResponse {
    log::info!("Received webhook data: {:?}", payload);

    (StatusCode::OK, Json(payload))
}
