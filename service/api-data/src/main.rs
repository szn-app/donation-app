// https://docs.rs/tower/latest/tower/trait.Service.html

mod model; 

use axum;
use env_logger;
use log;
use std::env;


#[tokio::main]
async fn main() {
    // Initialize logger with appropriate level based on environment
    let log_level = if cfg!(debug_assertions) {
        "debug"
    } else {
        "info"
    };
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or(log_level)).init();

    let app = axum::Router::new()
        .merge(model::routes())
        // Add a fallback route for handling 404 Not Found
        .fallback(handle_not_found);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:80").await.unwrap();
    log::info!("Server running on http://0.0.0.0:80");
    axum::serve(listener, app).await.unwrap();
}

// Handler for routes that don't match any defined routes
async fn handle_not_found() -> (axum::http::StatusCode, &'static str) {
    (axum::http::StatusCode::NOT_FOUND, "not found")
}
