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

pub fn routes() -> Router {
    Router::new()
        .route("/health/status", get(health_status))
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
