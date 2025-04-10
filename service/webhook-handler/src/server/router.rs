use axum::{
    extract::{Form, Query},
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::{get, post},
    Extension, Json, Router,
};
use log;
use std::env;

pub use crate::handlers::user_sync::webhook_kratos_registration_handler;

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
