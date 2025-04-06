// https://docs.rs/tower/latest/tower/trait.Service.html

mod handler;

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

    let app = handler::routes();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3010").await.unwrap();
    log::info!("Server running on http://0.0.0.0:3010");
    axum::serve(listener, app).await.unwrap();
}
