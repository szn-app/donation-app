use axum::{
    routing::{get, post},
    Extension, Router,
};
use log;
use std::env;

pub use crate::handler;

pub fn routes() -> Router {
    Router::new()
        .route("/health/status", get(handler::health_status))
        .route(
            "/auth/kratos/registration",
            post(handler::user_sync::webhook_kratos_registration_handler),
        )
        .route(
            "/auth/kratos/login",
            post(handler::webhook_kratos_handler_debug),
        )
        .fallback(handler::handler_404)
}
