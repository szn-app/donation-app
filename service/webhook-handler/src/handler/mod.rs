use axum::{
    extract::{Form, Query},
    http::StatusCode,
    response::{Html, IntoResponse},
    Json,
};

pub mod user_sync;

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
