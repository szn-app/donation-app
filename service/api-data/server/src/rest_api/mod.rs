pub mod handler;

use axum::{
    routing::{get, post},
    Router,
};

pub fn routes() -> axum::Router {
    Router::new()
        .route("/private/dummy_post_data", post(handler::dummy_post_data))
        .route("/public/dummy_get_data", get(handler::dummy_get_data))
}
