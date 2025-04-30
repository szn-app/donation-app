pub mod handler;

use axum;

pub fn routes() -> axum::Router {
    axum::Router::new()
        .route(
            "/dummy_post_data",
            axum::routing::post(handler::dummy_post_data),
        )
        .route(
            "/dummy_get_data",
            axum::routing::get(handler::dummy_get_data),
        )
}
