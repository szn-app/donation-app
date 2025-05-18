pub mod handler;

use crate::server::http::handle_not_found;
use axum::{
    routing::{get, post},
    Router,
};
use http;
use tower_http;

pub fn routes(app_endpoint: &str) -> axum::Router {
    // allow calling api subdomain from root domain
    let cors_layer = tower_http::cors::CorsLayer::new()
        // NOTE: wild card origin headers do not work well in all browsers
        .allow_origin(app_endpoint.parse::<http::header::HeaderValue>().unwrap())
        // NOTE: methods should be explicitely mentioned (tower_http::cors::Any won't work on browsers) when credentials are passed (e.g. Authorization headers)
        .allow_methods([
            http::Method::OPTIONS,
            http::Method::HEAD,
            http::Method::POST,
            http::Method::GET,
            http::Method::PUT,
            http::Method::DELETE,
            http::Method::PATCH,
            // http::Method::CONNECT,
            // http::Method::TRACE,
        ])
        .allow_headers(tower_http::cors::Any);

    Router::new()
        .route("/private/dummy_post_data", post(handler::dummy_post_data))
        .route("/public/dummy_get_data", get(handler::dummy_get_data))
        .fallback(handle_not_found)
        .layer(cors_layer)
}
